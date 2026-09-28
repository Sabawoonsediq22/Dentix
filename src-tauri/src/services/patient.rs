use crate::models::*;
use crate::services::errors::AppResult;
use chrono::Utc;
use sqlx::{SqlitePool, Transaction};
use uuid::Uuid;

pub struct PatientService;

impl PatientService {
    pub async fn list(
        pool: &SqlitePool,
        query: Option<&str>,
        gender: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> AppResult<PatientPageResult> {
        let offset = ((page.max(1) - 1) * per_page) as i64;
        let per_page_i64 = per_page as i64;

        let (total, items): (i64, Vec<Patient>) = if let Some(q_str) = query {
            if !q_str.trim().is_empty() {
                let like = format!("%{}%", q_str.trim());
                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM patients WHERE full_name LIKE ? OR phone LIKE ? OR id LIKE ?",
                )
                .bind(&like)
                .bind(&like)
                .bind(&like)
                .fetch_one(pool)
                .await?;
                let items: Vec<Patient> = sqlx::query_as(
                    "SELECT id, full_name, phone, age, gender, address, COALESCE((SELECT MAX(visit_date) FROM visits WHERE visits.patient_id = patients.id), '') as last_visit, created_at, updated_at FROM patients WHERE full_name LIKE ? OR phone LIKE ? OR id LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(&like)
                .bind(&like)
                .bind(&like)
                .bind(per_page_i64)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                (count, items)
            } else if let Some(g) = gender {
                if g != "All" {
                    let count: i64 = sqlx::query_scalar(
                        "SELECT COUNT(*) FROM patients WHERE gender = ?",
                    )
                    .bind(g)
                    .fetch_one(pool)
                    .await?;
                    let items: Vec<Patient> = sqlx::query_as(
                        "SELECT id, full_name, phone, age, gender, address, COALESCE((SELECT MAX(visit_date) FROM visits WHERE visits.patient_id = patients.id), '') as last_visit, created_at, updated_at FROM patients WHERE gender = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
                    )
                    .bind(g)
                    .bind(per_page_i64)
                    .bind(offset)
                    .fetch_all(pool)
                    .await?;
                    (count, items)
                } else {
                    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM patients")
                        .fetch_one(pool)
                        .await?;
                    let items: Vec<Patient> = sqlx::query_as(
                        "SELECT id, full_name, phone, age, gender, address, COALESCE((SELECT MAX(visit_date) FROM visits WHERE visits.patient_id = patients.id), '') as last_visit, created_at, updated_at FROM patients ORDER BY created_at DESC LIMIT ? OFFSET ?"
                    )
                    .bind(per_page_i64)
                    .bind(offset)
                    .fetch_all(pool)
                    .await?;
                    (count, items)
                }
            } else {
                let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM patients")
                    .fetch_one(pool)
                    .await?;
                let items: Vec<Patient> = sqlx::query_as(
                    "SELECT id, full_name, phone, age, gender, address, COALESCE((SELECT MAX(visit_date) FROM visits WHERE visits.patient_id = patients.id), '') as last_visit, created_at, updated_at FROM patients ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(per_page_i64)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                (count, items)
            }
        } else if let Some(g) = gender {
            if g != "All" {
                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM patients WHERE gender = ?",
                )
                .bind(g)
                .fetch_one(pool)
                .await?;
                let items: Vec<Patient> = sqlx::query_as(
                    "SELECT id, full_name, phone, age, gender, address, COALESCE((SELECT MAX(visit_date) FROM visits WHERE visits.patient_id = patients.id), '') as last_visit, created_at, updated_at FROM patients WHERE gender = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(g)
                .bind(per_page_i64)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                (count, items)
            } else {
                let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM patients")
                    .fetch_one(pool)
                    .await?;
                let items: Vec<Patient> = sqlx::query_as(
                    "SELECT id, full_name, phone, age, gender, address, COALESCE((SELECT MAX(visit_date) FROM visits WHERE visits.patient_id = patients.id), '') as last_visit, created_at, updated_at FROM patients ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(per_page_i64)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                (count, items)
            }
        } else {
            let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM patients")
                .fetch_one(pool)
                .await?;
            let items: Vec<Patient> = sqlx::query_as(
                "SELECT id, full_name, phone, age, gender, address, COALESCE((SELECT MAX(visit_date) FROM visits WHERE visits.patient_id = patients.id), '') as last_visit, created_at, updated_at FROM patients ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(per_page_i64)
            .bind(offset)
            .fetch_all(pool)
            .await?;
            (count, items)
        };

        let total_pages = ((total + per_page_i64 - 1) / per_page_i64).max(1);

        Ok(PatientPageResult {
            items,
            total,
            page,
            per_page,
            total_pages,
        })
    }

    pub async fn find(pool: &SqlitePool, id: &str) -> AppResult<Patient> {
        let patient = sqlx::query_as(
            "SELECT id, full_name, phone, age, gender, address, COALESCE((SELECT MAX(visit_date) FROM visits WHERE visits.patient_id = patients.id), '') as last_visit, created_at, updated_at FROM patients WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        patient.ok_or_else(|| {
            crate::services::errors::AppError::NotFound(format!("Patient {} not found", id))
        })
    }

    pub async fn create(pool: &SqlitePool, input: CreatePatientInput) -> AppResult<CreatedPatient> {
        let mut tx = pool.begin().await?;
        let id = format!("KD-{}-{:06}", Utc::now().format("%Y%m%d"), Uuid::new_v4().simple().to_string().chars().take(6).collect::<String>());

        let gender_str = match input.gender {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Other => "Other",
        };

        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO patients (id, full_name, phone, age, gender, address, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&input.full_name)
        .bind(&input.phone)
        .bind(input.age)
        .bind(gender_str)
        .bind(&input.address)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await?;

        Self::insert_allergies(&mut tx, &id, input.allergies.as_deref()).await?;
        Self::insert_medical_conditions(&mut tx, &id, input.medical_conditions.as_deref()).await?;
        let visit_id = Self::insert_visit(
            &mut tx,
            &id,
            input.visit_date.as_deref(),
            input.chief_complaint.as_deref(),
            &now,
        )
        .await?;

        let mut treatment_record_ids: Vec<String> = Vec::new();

        if let Some(procedures) = &input.procedures {
            for proc in procedures {
                if let Some(procedure_id) = Self::insert_procedure(
                    &mut tx,
                    &visit_id,
                    Some(proc.procedure_name.as_str()),
                    proc.procedure_additional_note.as_deref(),
                    Some(proc.procedure_price_afn),
                    Some(proc.procedure_price_usd),
                    &now,
                )
                .await?
                {
                    let tr_id = Self::insert_treatment_record(
                        &mut tx,
                        &visit_id,
                        &procedure_id,
                        Some(proc.number_of_procedures),
                        proc.treatment_teeth.as_deref(),
                    )
                    .await?;
                    treatment_record_ids.push(tr_id);
                }
            }
        }

        let discount_afn = input.discount_afn.unwrap_or(0.0);
        let discount_usd = input.discount_usd.unwrap_or(0.0);
        let invoice = Self::insert_invoice(
            &mut tx,
            &visit_id,
            input.paid_amount_afn,
            input.paid_amount_usd,
            discount_afn,
            discount_usd,
        )
        .await?;

        tx.commit().await?;

        Ok(CreatedPatient {
            id,
            full_name: input.full_name,
            phone: input.phone,
            age: input.age,
            gender: input.gender,
            address: input.address,
            last_visit: input.visit_date,
            created_at: now.clone(),
            updated_at: now,
            visit_id,
            invoice_id: invoice.id,
            invoice_number: invoice.invoice_number,
            treatment_record_id: treatment_record_ids.into_iter().next(),
        })
    }

    async fn insert_allergies(
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        patient_id: &str,
        allergies: Option<&str>,
    ) -> AppResult<()> {
        for allergy in Self::split_csv(allergies) {
            sqlx::query(
                "INSERT OR IGNORE INTO patient_allergies (patient_id, allergy_name) VALUES (?, ?)",
            )
            .bind(patient_id)
            .bind(allergy)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    async fn insert_medical_conditions(
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        patient_id: &str,
        medical_conditions: Option<&[String]>,
    ) -> AppResult<()> {
        for condition in Self::dedupe_strings(medical_conditions.unwrap_or(&[])) {
            sqlx::query(
                "INSERT OR IGNORE INTO medical_conditions (patient_id, condition_name, is_active) VALUES (?, ?, TRUE)",
            )
            .bind(patient_id)
            .bind(condition)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    async fn insert_visit(
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        patient_id: &str,
        visit_date: Option<&str>,
        chief_complaint: Option<&str>,
        now: &str,
    ) -> AppResult<String> {
        let visit_id = format!("V-{}-{:06}", Utc::now().format("%Y%m%d"), Uuid::new_v4().simple().to_string().chars().take(6).collect::<String>());
        let visit_date = Self::trimmed_optional(visit_date).unwrap_or_else(|| now.to_string());
        let chief_complaint = Self::trimmed_optional(chief_complaint);

        sqlx::query(
            "INSERT INTO visits (id, patient_id, visit_date, chief_complaint, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&visit_id)
        .bind(patient_id)
        .bind(visit_date)
        .bind(chief_complaint)
        .bind("Open")
        .bind(now)
        .bind(now)
        .execute(&mut **tx)
        .await?;

        Ok(visit_id)
    }

    async fn insert_procedure(
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        visit_id: &str,
        name: Option<&str>,
        additional_note: Option<&str>,
        price_afn: Option<f64>,
        price_usd: Option<f64>,
        now: &str,
    ) -> AppResult<Option<String>> {
        let Some(name) = Self::trimmed_optional(name) else {
            return Ok(None);
        };
        let price_afn = price_afn.filter(|p| *p >= 0.0).unwrap_or(0.0);
        let price_usd = price_usd.filter(|p| *p >= 0.0).unwrap_or(0.0);
        let additional_note = Self::trimmed_optional(additional_note);
        let id = format!("PROC-{}", Uuid::new_v4().simple());

        sqlx::query(
            "INSERT INTO procedures (id, visit_id, name, additional_note, procedure_price_afn, procedure_price_usd, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(visit_id)
        .bind(&name)
        .bind(additional_note)
        .bind(price_afn)
        .bind(price_usd)
        .bind(now)
        .bind(now)
        .execute(&mut **tx)
        .await?;

        Ok(Some(id))
    }

    async fn insert_treatment_record(
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        visit_id: &str,
        procedure_id: &str,
        number_of_procedures: Option<i32>,
        treatment_teeth: Option<&[TreatmentToothInput]>,
    ) -> AppResult<String> {
        let number_of_procedures = number_of_procedures.unwrap_or(1).max(1);
        let treatment_teeth = treatment_teeth
            .unwrap_or(&[])
            .iter()
            .map(|tooth| TreatmentToothInput {
                tooth_number: tooth.tooth_number,
                tooth_quadrant: tooth.tooth_quadrant.trim().to_string(),
            })
            .filter(|tooth| tooth.tooth_number > 0 && !tooth.tooth_quadrant.is_empty())
            .collect::<Vec<_>>();

        let id = format!("TR-{}", Uuid::new_v4().simple());
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO treatment_records (id, visit_id, procedure_id, number_of_procedures, performed_at)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(visit_id)
        .bind(procedure_id)
        .bind(number_of_procedures)
        .bind(&now)
        .execute(&mut **tx)
        .await?;

        for tooth in treatment_teeth.iter() {
            sqlx::query(
                "INSERT INTO treatment_tooth (id, treatment_record_id, tooth_number, tooth_quadrant) VALUES (NULL, ?, ?, ?)",
            )
            .bind(&id)
            .bind(tooth.tooth_number)
            .bind(tooth.tooth_quadrant.trim())
            .execute(&mut **tx)
            .await?;
        }

        Ok(id)
    }

    #[allow(unused_variables)]
    async fn insert_invoice(
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        visit_id: &str,
        paid_amount_afn: f64,
        paid_amount_usd: f64,
        discount_afn: f64,
        discount_usd: f64,
    ) -> AppResult<Invoice> {
        let row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(p.procedure_price_afn * tr.number_of_procedures), 0.0),
               COALESCE(SUM(p.procedure_price_usd * tr.number_of_procedures), 0.0)
             FROM treatment_records tr
             JOIN procedures p ON p.id = tr.procedure_id
             WHERE tr.visit_id = ?",
        )
        .bind(visit_id)
        .fetch_one(&mut **tx)
        .await?;

        let subtotal_afn = row.0;
        let subtotal_usd = row.1;

        let total_afn = (subtotal_afn - discount_afn).max(0.0);
        let total_usd = (subtotal_usd - discount_usd).max(0.0);
        let outstanding_afn = total_afn - paid_amount_afn;
        let outstanding_usd = total_usd - paid_amount_usd;

        let paid_afn_total = if outstanding_afn < 0.0 { total_afn } else { paid_amount_afn.min(total_afn) };
        let paid_usd_total = if outstanding_usd < 0.0 { total_usd } else { paid_amount_usd.min(total_usd) };

        let final_outstanding_afn = (total_afn - paid_afn_total).max(0.0);
        let final_outstanding_usd = (total_usd - paid_usd_total).max(0.0);

        let status = if final_outstanding_afn == 0.0 && final_outstanding_usd == 0.0 {
            InvoiceStatus::Paid
        } else if paid_afn_total > 0.0 || paid_usd_total > 0.0 {
            InvoiceStatus::Partial
        } else {
            InvoiceStatus::Unpaid
        };

        let invoice_id = format!("INV-{}", uuid::Uuid::new_v4().simple());
        let invoice_number = format!("INV-{}", Utc::now().timestamp_millis());
        let now = Utc::now().to_rfc3339();

        let invoice = sqlx::query_as::<_, Invoice>(
            "INSERT INTO invoices (id, visit_id, invoice_number,
              subtotal_afn, subtotal_usd,
              discount_afn, discount_usd,
              total_afn, total_usd,
              paid_afn, paid_usd,
              outstanding_afn, outstanding_usd,
              status, issued_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             RETURNING id, visit_id, invoice_number,
               subtotal_afn, subtotal_usd,
               discount_afn, discount_usd,
               total_afn, total_usd,
               paid_afn, paid_usd,
               outstanding_afn, outstanding_usd,
               status, issued_at"
        )
        .bind(&invoice_id)
        .bind(visit_id)
        .bind(&invoice_number)
        .bind(subtotal_afn)
        .bind(subtotal_usd)
        .bind(discount_afn)
        .bind(discount_usd)
        .bind(total_afn)
        .bind(total_usd)
        .bind(paid_afn_total)
        .bind(paid_usd_total)
        .bind(final_outstanding_afn)
        .bind(final_outstanding_usd)
        .bind(match status {
            InvoiceStatus::Unpaid => "Unpaid",
            InvoiceStatus::Partial => "Partial",
            InvoiceStatus::Paid => "Paid",
        })
        .bind(&now)
        .fetch_one(&mut **tx)
        .await?;

        if paid_afn_total > 0.0 || paid_usd_total > 0.0 {
            sqlx::query(
                "INSERT INTO payments (id, invoice_id, amount_afn, amount_usd, method, notes, received_at)
                 VALUES (?, ?, ?, ?, 'Cash', '', ?)",
            )
            .bind(format!("PAY-{}", uuid::Uuid::new_v4().simple()))
            .bind(&invoice_id)
            .bind(paid_afn_total)
            .bind(paid_usd_total)
            .bind(&now)
            .execute(&mut **tx)
            .await?;
        }

        Ok(invoice)
    }

    fn split_csv(value: Option<&str>) -> Vec<String> {
        let mut result = Vec::new();

        for item in value
            .unwrap_or("")
            .split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
        {
            if !result
                .iter()
                .any(|existing: &String| existing.eq_ignore_ascii_case(&item))
            {
                result.push(item);
            }
        }

        result
    }

    fn dedupe_strings(values: &[String]) -> Vec<String> {
        let mut result = Vec::new();

        for value in values
            .iter()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
        {
            if !result
                .iter()
                .any(|existing: &String| existing.eq_ignore_ascii_case(&value))
            {
                result.push(value);
            }
        }

        result
    }

    fn trimmed_optional(value: Option<&str>) -> Option<String> {
        value
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    }

    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        input: UpdatePatientInput,
    ) -> AppResult<Patient> {
        let existing = Self::find(pool, id).await?;

        let full_name = input
            .full_name
            .unwrap_or_else(|| existing.full_name.clone());
        let phone = input.phone.unwrap_or_else(|| existing.phone.clone());
        let age = input.age.unwrap_or(existing.age);
        let gender = input.gender.unwrap_or(existing.gender);
        let address = input.address;

        let gender_str = match gender {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Other => "Other",
        };

        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE patients SET full_name=?, phone=?, age=?, gender=?, address=?, updated_at=? WHERE id=?",
        )
        .bind(&full_name)
        .bind(&phone)
        .bind(age)
        .bind(gender_str)
        .bind(&address)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(Patient {
            id: existing.id,
            full_name,
            phone,
            age,
            gender,
            address,
            last_visit: existing.last_visit,
            created_at: existing.created_at,
            updated_at: now,
        })
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM patients WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(crate::services::errors::AppError::NotFound(format!(
                "Patient {} not found",
                id
            )));
        }
        Ok(())
    }

    pub async fn add_medical_condition(
        pool: &SqlitePool,
        patient_id: &str,
        condition_name: &str,
    ) -> AppResult<()> {
        sqlx::query("INSERT INTO medical_conditions (patient_id, condition_name, is_active) VALUES (?, ?, TRUE)")
            .bind(patient_id)
            .bind(condition_name)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_medical_info(pool: &SqlitePool, id: &str) -> AppResult<PatientMedicalInfoResponse> {
        let allergies: Vec<String> = sqlx::query_scalar("SELECT allergy_name FROM patient_allergies WHERE patient_id = ?")
            .bind(id)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r: String| r)
            .collect();

        let conditions: Vec<String> = sqlx::query_scalar("SELECT condition_name FROM medical_conditions WHERE patient_id = ?")
            .bind(id)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r: String| r)
            .collect();

        Ok(PatientMedicalInfoResponse {
            allergies,
            medical_conditions: conditions,
        })
    }

    pub async fn get_statistics(pool: &SqlitePool, id: &str) -> AppResult<PatientStatisticsResponse> {
        let row: (f64, f64, f64, f64, f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(i.total_afn + i.total_usd), 0.0),
               COALESCE(SUM(i.total_afn), 0.0),
               COALESCE(SUM(i.total_usd), 0.0),
               COALESCE(SUM(i.outstanding_afn + i.outstanding_usd), 0.0),
               COALESCE(SUM(i.outstanding_afn), 0.0),
               COALESCE(SUM(i.outstanding_usd), 0.0)
             FROM invoices i
             JOIN visits v ON v.id = i.visit_id
             WHERE v.patient_id = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        let last_visit: Option<(String, String)> = sqlx::query_as(
            "SELECT v.visit_date, p.name FROM visits v JOIN invoices i ON i.visit_id = v.id JOIN procedures p ON p.visit_id = v.id WHERE v.patient_id = ? ORDER BY v.visit_date DESC LIMIT 1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(PatientStatisticsResponse {
            total_spent: row.0,
            total_spent_afn: row.1,
            total_spent_usd: row.2,
            last_visit_date: last_visit.as_ref().map(|(d, _)| d.clone()),
            last_visit_procedure: last_visit.map(|(_, p)| p),
            outstanding_balance: row.3,
            outstanding_balance_afn: row.4,
            outstanding_balance_usd: row.5,
        })
    }

    pub async fn update_medical_info(
        pool: &SqlitePool,
        patient_id: &str,
        input: UpdatePatientMedicalInfoInput,
    ) -> AppResult<()> {
        let mut tx = pool.begin().await?;

        // Clear existing records
        sqlx::query("DELETE FROM patient_allergies WHERE patient_id = ?")
            .bind(patient_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM medical_conditions WHERE patient_id = ?")
            .bind(patient_id)
            .execute(&mut *tx)
            .await?;

        // Insert new allergies
        for allergy in Self::split_csv(input.allergies.as_deref()) {
            sqlx::query("INSERT INTO patient_allergies (patient_id, allergy_name) VALUES (?, ?)")
                .bind(patient_id)
                .bind(&allergy)
                .execute(&mut *tx)
                .await?;
        }

        // Insert new conditions
        for condition in Self::dedupe_strings(input.medical_conditions.as_deref().unwrap_or(&[])) {
            sqlx::query("INSERT INTO medical_conditions (patient_id, condition_name, is_active) VALUES (?, ?, TRUE)")
                .bind(patient_id)
                .bind(&condition)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

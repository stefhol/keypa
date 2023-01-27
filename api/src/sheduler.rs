use entities::model::{tbl_request, tbl_user};
use job_scheduler::{Job, JobScheduler};
use sea_orm::{DatabaseConnection, DbBackend, EntityTrait, Statement};
use std::{thread, time::Duration};
use tokio::runtime;
use tracing::error;
use uuid::Uuid;

use crate::util::mail::{send_mail, Email};

pub fn start(db: DatabaseConnection) {
    let email_shedule = match dotenv::var("EMAIL_SHEDULE") {
        Ok(val) => val,
        Err(_) => String::from("/15 * * * * *"),
    };
    thread::spawn(move || {
        let db1 = db.clone();
        let shedule = email_shedule.clone();
        let mut shed = JobScheduler::new();
        shed.add(Job::new(shedule.parse().unwrap(), move || {
            let db = db1.clone();
            let rt = runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async move {
                email_on_keycard(&db).await;
            });
        }));
        let db2 = db.clone();
        // run every hour
        shed.add(Job::new("0 * * * *".parse().unwrap(), move || {
            let db = db2.clone();
            let rt = runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async move {
                cleanup(&db).await;
            });
        }));
        loop {
            shed.tick();
            thread::sleep(Duration::from_millis(500));
        }
    });
}

async fn cleanup(db: &DatabaseConnection) {
    let requests = tbl_request::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
        select * from tbl_request
        where active_until <  timezone('utc', now())
        "#,
            vec![],
        ))
        .all(db)
        .await;
    if let Ok(requests) = requests {
        for request in &requests {
            match crate::crud::request::change::move_to_archive(
                &Uuid::default(),
                db,
                &request.request_id,
            )
            .await
            {
                Ok(_) => {}
                Err(err) => return error!("{err}"),
            }
            if let Some(keycard) = request.keycard_id {
                match crate::crud::keycard::move_to_archive(db, &Uuid::default(), &keycard).await {
                    Ok(_) => {}
                    Err(err) => return error!("{err}"),
                }
            }
        }
    }
}

async fn email_on_keycard(db: &DatabaseConnection) {
    let query = tbl_user::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            select tu.* from tbl_request tr
            join tbl_keycard tk on tr.keycard_id = tk.keycard_id
            join tbl_user tu on tk.user_id = tu.user_id
            where given_out is not null
            and is_given_back = false
            and active_until < timezone('utc', now() + interval '1 week');
            "#,
            vec![],
        ))
        .all(&db.clone())
        .await;
    if let Ok(query) = query {
        for user in query {
            match send_mail(Email{
                email_to:user.email.to_owned(),
                subject:String::from("Keycard läuft bald ab"),
                message:String::from("Eine Keycard läuft bald ab. Bitte senden Sie die Karte an das Gebäude der Verwaltung")
            }){
                Ok(()) => {},
                Err(e) => error!("{e}"),
            };
        }
    }
}

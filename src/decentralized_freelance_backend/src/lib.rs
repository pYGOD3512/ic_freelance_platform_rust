use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct User {
    id: String,
    reputation: u32,
}

#[derive(CandidType, Deserialize, Clone)]
struct Job {
    id: String,
    title: String,
    description: String,
    budget: u32,
    client: String,
    freelancer: Option<String>,
    status: JobStatus,
}

#[derive(CandidType, Deserialize, Clone)]
enum JobStatus {
    Open,
    Assigned,
    Completed,
    Disputed,
}

type UserStore = HashMap<String, User>;
type JobStore = HashMap<String, Job>;

thread_local! {
    static USERS: std::cell::RefCell<UserStore> = std::cell::RefCell::new(HashMap::new());
    static JOBS: std::cell::RefCell<JobStore> = std::cell::RefCell::new(HashMap::new());
}

#[update]
fn register_user(id: String) -> Result<(), String> {
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if users.contains_key(&id) {
            Err("User already exists".to_string())
        } else {
            users.insert(id.clone(), User { id, reputation: 0 });
            Ok(())
        }
    })
}

#[update]
fn post_job(client: String, title: String, description: String, budget: u32) -> Result<String, String> {
    let job_id = ic_cdk::api::time().to_string(); // Using timestamp as a simple unique ID
    let job = Job {
        id: job_id.clone(),
        title,
        description,
        budget,
        client,
        freelancer: None,
        status: JobStatus::Open,
    };
    
    JOBS.with(|jobs| {
        jobs.borrow_mut().insert(job_id.clone(), job);
    });
    
    Ok(job_id)
}

#[query]
fn get_job(job_id: String) -> Option<Job> {
    JOBS.with(|jobs| jobs.borrow().get(&job_id).cloned())
}

#[query]
fn list_open_jobs() -> Vec<Job> {
    JOBS.with(|jobs| {
        jobs.borrow()
            .values()
            .filter(|job| matches!(job.status, JobStatus::Open))
            .cloned()
            .collect()
    })
}
Here’s your formatted spec following the style of your /contact example:


---

Web routes

/confirmation -> handler_confirmation_page -> Job Round Confirmation Page


---

REST routes

File: /site/confirmation.rs
Base URL: 'api/site/'

Handlers:

handler_round_list
-> /confirmation/round-list
-> GET

handler_accept_round
-> /confirmation/{job_id}/{round_id}/{candidate_id}/accept
-> POST

handler_reject_round
-> /confirmation/{job_id}/{round_id}/{candidate_id}/reject
-> POST

handler_request_reschedule
-> /confirmation/{job_id}/{round_id}/{candidate_id}/reschedule
-> POST



---

Structures

CandidateJobRound

title

description

status

scheduled_on

rescheduled_on (optional)



---

ContentAPI

File: api/site/confirmation.rs

get_round_list(candidate_id, job_id)
Output: Vec<CandidateJobRound>

> Get only job rounds from candidate_job_round, include reschedule info if status is requested.



accept_round(candidate_id, job_id, round_id)
Output: bool

> Updates candidate_job_round.status from pending -> accepted.



reject_round(candidate_id, job_id, round_id)
Output: bool

> Updates candidate_job_round.status to rejected and candidate job status to rejected.



request_reschedule(candidate_id, job_id, round_id)
Output: Result<bool, String>

> Updates candidate_job_round.status to requested, creates record in candidate_job_round_reschedule, increments requested_count.
Returns error if requested_count > 3: "Reschedule limit exceeded, User cannot reschedule".





---

Let me know if you want the DB schema or types added as Rust structs or in SQL.


CRINSERT INTO customer (name, mobile, email, company, address, description, tags, is_archive)
VALUES 
    ('John Doe', '1234567890', 'john.doe@example.com', 'ABC Company', '123 Main St, City A, Country X', 'Regular customer', 'regular, important', false),
    ('Jane Smith', '9876543210', 'jane.smith@example.com', 'XYZ Corporation', '456 Elm St, City B, Country Y', 'VIP customer', 'VIP, important', false),
    ('Alice Johnson', '5551112222', 'alice.johnson@example.com', '123 Industries', '789 Oak St, City C, Country Z', 'New customer', 'new', false),
    ('Bob Brown', '9998887777', 'bob.brown@example.com', 'Tech Solutions Inc.', '321 Pine St, City D, Country X', 'Returning customer', 'returning, regular', false),
    ('Emma Davis', '7776665555', 'emma.davis@example.com', 'Global Innovations', '654 Cedar St, City E, Country Y', 'Frequent buyer', 'frequent, regular', false),
    ('Michael Wilson', '3332221111', 'michael.wilson@example.com', 'Software Co.', '987 Maple St, City F, Country Z', 'Preferred customer', 'preferred, regular', false),
    ('Sarah Lee', '1112223333', 'sarah.lee@example.com', 'Tech Solutions Inc.', '234 Birch St, City G, Country X', 'First-time buyer', 'new', false),
    ('David Martinez', '4445556666', 'david.martinez@example.com', 'Global Innovations', '876 Walnut St, City H, Country Y', 'Loyal customer', 'loyal, regular', false),
    ('Emily Taylor', '6667778888', 'emily.taylor@example.com', '123 Industries', '543 Pine St, City I, Country Z', 'Satisfied customer', 'satisfied, regular', false),
    ('Chris Rodriguez', '8889990000', 'chris.rodriguez@example.com', 'XYZ Corporation', '765 Oak St, City J, Country X', 'Valued customer', 'valued, regular', false);


CREATE TABLE employees (
    id INT PRIMARY KEY,
    name VARCHAR(255),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    initial CHAR(1),
    gender VARCHAR(10),
    designation VARCHAR(255),
    degree VARCHAR(255),
    email VARCHAR(255),
    blood_group VARCHAR(10),
    marital_status VARCHAR(20),
    father_name VARCHAR(255),
    mother_name VARCHAR(255),
    city VARCHAR(100),
    current_address VARCHAR(255),
    permanent_address VARCHAR(255),
    education_details TEXT,
    experience_details TEXT,
    health_details TEXT,
    hobbies TEXT,
    description TEXT,
    is_active BOOLEAN,
    is_trainee BOOLEAN,
    joined_date DATE,
    exit_date DATE,
    mobile VARCHAR(20),
    contact_other VARCHAR(20),
    contact_mother VARCHAR(20),
    contact_father VARCHAR(20),
    image BLOB,
    created_by VARCHAR(255),
    created_by_id INT
);


INSERT INTO employee (
    initial, gender, designation, name, first_name, last_name, degree, email, blood_group, marital_status, father_name, mother_name, city, current_address, permanent_address, education_details, experience_details, health_details, hobbies, description, is_active, is_trainee, joined_date, exit_date, mobile, contact_other, contact_mother, contact_father, image, created_by, created_by_id
) VALUES 
    ('Mr.', 'Male', 'Manager', 'John Doe', 'John', 'Doe', 'MBA', 'john.doe@example.com', 'O+', 'Single', 'David Doe', 'Mary Doe', 'New York', '123 Main St', '456 Elm St', 'Bachelor in Business Administration from XYZ University', '5 years as a manager at ABC Corp', 'Good health condition', 'Reading, hiking', NULL, true, false, '2024-02-29', NULL, '1234567890', NULL, '0987654321', '0987654321', 'image_url_here', 'Admin', 1),
    ('Ms.', 'Female', 'Software Engineer', 'Jane Smith', 'Jane', 'Smith', 'BSc', 'jane.smith@example.com', 'A-', 'Married', 'Michael Smith', 'Sarah Smith', 'Los Angeles', '789 Oak St', '987 Pine St', 'Bachelor of Science in Computer Science from ABC University', '3 years as a software engineer at XYZ Corp', 'No major health issues', 'Playing guitar, swimming', NULL, true, false, '2023-12-15', NULL, '9876543210', NULL, '0123456789', '0123456789', 'image_url_here', 'Admin', 1),
    ('Mr.', 'Male', 'HR Manager', 'Michael Johnson', 'Michael', 'Johnson', 'MA', 'michael.johnson@example.com', 'B+', 'Divorced', 'Robert Johnson', 'Emily Johnson', 'Chicago', '456 Maple St', '321 Oak St', 'Master of Arts in Human Resource Management from DEF University', '8 years as an HR manager at LMN Corp', 'Occasional back pain', 'Cooking, gardening', NULL, true, false, '2022-10-20', NULL, '4567890123', NULL, '9876543210', '9876543210', 'image_url_here', 'Admin', 1),
    ('Mrs.', 'Female', 'Marketing Specialist', 'Emily Brown', 'Emily', 'Brown', 'BBA', 'emily.brown@example.com', 'AB-', 'Married', 'James Brown', 'Olivia Brown', 'San Francisco', '789 Cedar St', '654 Walnut St', 'Bachelor of Business Administration in Marketing from GHI University', '4 years as a marketing specialist at OPQ Corp', 'Allergy to pollen', 'Painting, yoga', NULL, true, false, '2023-05-10', NULL, '7890123456', NULL, '6543210987', '6543210987', 'image_url_here', 'Admin', 1),
    ('Mr.', 'Male', 'Accountant', 'Daniel Wilson', 'Daniel', 'Wilson', 'BCom', 'daniel.wilson@example.com', 'A+', 'Single', 'Richard Wilson', 'Sophia Wilson', 'Houston', '123 Elm St', '876 Oak St', 'Bachelor of Commerce in Accounting from JKL University', '2 years as an accountant at RST Corp', 'No health issues', 'Playing football, watching movies', NULL, true, false, '2024-01-05', NULL, '9012345678', NULL, '5432109876', '5432109876', 'image_url_here', 'Admin', 1);

INSERT INTO employee_auth (
    employee_id, username, password
) VALUES 
    (1, 'johndoe', 'hashed_password_here'),
    (2, 'janesmith', 'hashed_password_here'),
    (3, 'michaeljohnson', 'hashed_password_here'),
    (4, 'emilybrown', 'hashed_password_here'),
    (5, 'danielwilson', 'hashed_password_here');



SELECT r.title AS role, p.title AS privilege
FROM employee e
INNER JOIN employee_role er ON e.id = er.employee_id
INNER JOIN role r ON er.role_id = r.id
INNER JOIN role_privilege rp ON r.id = rp.role_id
INNER JOIN privilege p ON rp.privilege_id = p.id
WHERE e.username = 'employee_name';





INSERT INTO staff (name, mobile, gender, father_name, mother_name, address, description, is_active) 
VALUES 
('Mark Johnson', '123-456-7890', 'M', 'Michael Johnson', 'Sarah Johnson', '123 Main St, Anytown, USA', 'Senior Engineer', true),
('Emily Anderson', '987-654-3210', 'F', 'Robert Anderson', 'Emily Anderson', '456 Elm St, Othertown, USA', 'Marketing Manager', true),
('Peter Williams', '555-123-4567', 'M', 'James Williams', 'Jennifer Williams', '789 Oak St, Anycity, USA', 'HR Coordinator', true),
('Anna Thompson', '111-222-3333', 'F', 'William Thompson', 'Mary Thompson', '101 Pine St, Somewhere, USA', 'Software Developer', true),
('Thomas Clark', '777-888-9999', 'M', 'Thomas Clark', 'Patricia Clark', '202 Maple St, Anyville, USA', 'Project Manager', true),
('Olivia Lee', '444-555-6666', 'F', 'Daniel Lee', 'Linda Lee', '303 Walnut St, Anothercity, USA', 'Graphic Designer', true),
('Henry Rodriguez', '999-888-7777', 'M', 'Richard Rodriguez', 'Susan Rodriguez', '404 Oak St, Anytown, USA', 'Accountant', true),
('Sophia Garcia', '222-333-4444', 'F', 'John Garcia', 'Karen Garcia', '505 Cedar St, Anycity, USA', 'Sales Representative', true),
('Ethan Martinez', '666-777-8888', 'M', 'Jose Martinez', 'Maria Martinez', '606 Pine St, Anycity, USA', 'Customer Service Specialist', true),
('Isabella Davis', '333-222-1111', 'F', 'David Davis', 'Laura Davis', '707 Elm St, Somewhere, USA', 'Operations Manager', true);


-- Inserting data into ticket table
INSERT INTO ticket (title, description, status, day_on, resolution) VALUES 
('Software Bug', 'Application crashes on startup', 'Open', '2024-04-15', NULL),
('Network Issue', 'Unable to connect to the server', 'Open', '2024-04-14', NULL),
('Login Problem', 'Users cannot log in to the system', 'Open', '2024-04-13', NULL),
('Database Error', 'SQL error when querying data', 'Closed', '2024-04-12', 'Fixed the SQL query'),
('UI Glitch', 'Button not displaying correctly', 'Open', '2024-04-11', NULL),
('Performance Issue', 'System response time is slow', 'Open', '2024-04-10', NULL),
('Email Notification', 'Emails are not being sent', 'Open', '2024-04-09', NULL),
('Security Concern', 'Possible data breach detected', 'Open', '2024-04-08', NULL),
('Feature Request', 'New feature for customer feedback', 'Open', '2024-04-07', NULL),
('Server Maintenance', 'Scheduled server downtime', 'Closed', '2024-04-06', 'Completed server maintenance');

-- Inserting data into ticket_comment table
INSERT INTO ticket_comment (ticket_id, created_by_id, created_by, content) VALUES
(1, 101, 'JohnDoe', 'Investigating the issue'),
(2, 102, 'JaneDoe', 'Checking network configurations'),
(3, 103, 'AliceSmith', 'Resetting user passwords'),
(4, 104, 'BobJohnson', 'Optimized SQL query for performance'),
(5, 105, 'CharlieBrown', 'Fixing CSS for the button display'),
(6, 106, 'DavidWilson', 'Monitoring system resources'),
(7, 107, 'EvaGreen', 'Reviewing email server logs'),
(8, 108, 'FrankTaylor', 'Initiating security audit'),
(9, 109, 'GraceClark', 'Gathering user feedback for the feature'),
(10, 110, 'HelenWhite', 'Completed server maintenance successfully');


-- Inserting data into the project table
INSERT INTO project (title, slug, code, type, description, is_archive)
VALUES ('Project 1', 'project-1', 'P001', 'Research', 'Description of Project 1', false);

INSERT INTO project (title, slug, code, type, description, is_archive)
VALUES ('Project 2', 'project-2', 'P002', 'Development', 'Description of Project 2', false);

INSERT INTO project (title, slug, code, type, description, is_archive)
VALUES ('Project 3', 'project-3', 'P003', 'Design', 'Description of Project 3', false);

-- Inserting data into the project_member table
INSERT INTO project_member (project_id, employee_id, role)
VALUES (1, 1, 'Project Manager');

INSERT INTO project_member (project_id, employee_id, role)
VALUES (1, 2, 'Developer');

INSERT INTO project_member (project_id, employee_id, role)
VALUES (2, 3, 'Designer');

INSERT INTO project_member (project_id, employee_id, role)
VALUES (2, 1, 'Project Manager');

INSERT INTO project_member (project_id, employee_id, role)
VALUES (3, 2, 'Developer');

INSERT INTO project_member (project_id, employee_id, role)
VALUES (3, 3, 'Designer');

INSERT INTO project_member (project_id, employee_id, role)
VALUES (3, 1, 'Project Manager');


INSERT INTO team_link (team_id, title, tags, link)
VALUES
    (1, 'Team A Homepage', '#homepage#teamA', 'http://example.com/team-a'),
    (2, 'Team B Homepage', '#homepage#teamB', 'http://example.com/team-b'),
    (3, 'Team C Homepage', '#homepage#teamC', 'http://example.com/team-c'),
    (4, 'Team D Homepage', '#homepage#teamD', 'http://example.com/team-d'),
    (5, 'Team E Homepage', '#homepage#teamE', 'http://example.com/team-e'),
    (6, 'Team F Homepage', '#homepage#teamF', 'http://example.com/team-f'),
    (7, 'Team G Homepage', '#homepage#teamG', 'http://example.com/team-g'),
    (8, 'Team H Homepage', '#homepage#teamH', 'http://example.com/team-h'),
    (9, 'Team I Homepage', '#homepage#teamI', 'http://example.com/team-i'),
    (10, 'Team J Homepage', '#homepage#teamJ', 'http://example.com/team-j');

use std::net::SocketAddr;

use axum::{routing::get, Json};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(openapi))]
struct ApiDoc;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() {
    let socket_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let app = axum::Router::new().route("/api-docs/openapi.json", get(openapi));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}


use aes_gcm::{
    aead::{Aad, NewAead, SealingKey},
    Aes256Gcm, KeySize, Nonce,
};
use rand::rngs::OsRng;

// Function to encrypt data
fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let password_bytes = password.as_bytes();
    let key = KeySize::Key256.into(password_bytes);
    let cipher = Aes256Gcm::new(key)?;
    let nonce = Nonce::generate(&mut OsRng);

    let encrypted_data = cipher.seal(nonce.clone(), Aad::none(), data)?;

    Ok(vec![nonce.as_bytes().to_vec(), encrypted_data])
}

// Function to decrypt data
fn decrypt(ciphertext: Vec<u8>, password: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let password_bytes = password.as_bytes();
    let key = KeySize::Key256.into(password_bytes);
    let cipher = Aes256Gcm::new(key)?;
    let (nonce, encrypted_data) = ciphertext.split_at(12).map(|slice| slice.to_vec());

    let decrypted_data = cipher.open(nonce, Aad::none(), encrypted_data)?;

    Ok(decrypted_data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = b"Hello, World!";
    let password = "mysecretpassword";

    // Encrypting data
    let encrypted = encrypt(data, password)?;
    println!("Encrypted data: {:?}", encrypted);

    // Decrypting data
    let decrypted = decrypt(encrypted, password)?;
    println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));

    Ok(())
}






        INSERT INTO Workspaces (title, description) VALUES
('Project Management', 'Task tracking, Team collaboration, Timeline and milestones'),
('Customer Relationship Management (CRM)', 'Contact management, Sales pipeline tracking, Email and call logs'),
('Inventory Management', 'Product catalog, Stock levels, Supplier information'),
('Event Planning', 'Event schedules, Attendee lists, Budget management'),
('Human Resources', 'Employee database, Leave and attendance tracking, Performance reviews'),
('Content Management System (CMS)', 'Blog post scheduling, Content drafts and approvals, Media library'),
('Finance and Accounting', 'Expense tracking, Invoice generation, Budgeting'),
('Help Desk and Support', 'Ticketing system, Knowledge base, Customer feedback'),
('Education and E-learning', 'Course management, Student progress tracking, Assignment submissions'),
('E-commerce Management', 'Order tracking, Customer database, Product listings');


        -- Insert sample surveys
INSERT INTO survey (title, created_by, published_on, no_attended, no_total, status)
VALUES 
    ('Employee Engagement Survey', 'manager1', '2024-08-01', 50, 100, 'Published'),
    ('Health & Wellness Survey', 'hr_admin', '2024-08-05', 20, 50, 'Published'),
    ('Remote Work Feedback', 'manager2', '2024-08-10', 75, 80, 'Published'),
    ('Team Satisfaction Survey', 'manager3', '2024-08-15', 40, 50, 'Published'),
    ('Company Culture Survey', 'hr_admin', '2024-08-20', 10, 150, 'Draft'),
    ('Training Effectiveness', 'trainer1', '2024-08-22', 60, 80, 'Published'),
    ('Leadership Feedback Survey', 'leader1', '2024-08-24', 30, 50, 'Published'),
    ('Productivity Check', 'manager4', '2024-08-26', 45, 75, 'Published'),
    ('Client Interaction Survey', 'sales_lead', '2024-08-28', 35, 60, 'Published'),
    ('Work-Life Balance Survey', 'hr_admin', '2024-08-30', 25, 100, 'Draft');

-- Insert sample survey options
INSERT INTO survey_option (survey_id, answer, no_votes)
VALUES 
    (1, 'Strongly Agree', 20),
    (1, 'Agree', 30),
    (2, 'Yes', 15),
    (2, 'No', 5),
    (3, 'Remote', 50),
    (3, 'Hybrid', 25),
    (4, 'Very Satisfied', 25),
    (4, 'Satisfied', 15),
    (5, 'Yes', 7),
    (5, 'No', 3);

-- Insert sample survey_employee responses
INSERT INTO survey_employee (survey_id, employee_id, option_id)
VALUES 
    (1, 101, 1),
    (1, 102, 2),
    (2, 103, 3),
    (2, 104, 4),
    (3, 105, 5),
    (3, 106, 6),
    (4, 107, 7),
    (4, 108, 8),


        -- Insert sample surveys with content and notes
INSERT INTO survey (title, created_by, published_on, no_attended, no_total, status, content, notes)
VALUES 
    ('Employee Engagement Survey', 'manager1', '2024-08-01', 50, 100, 'Published', 
    'This survey is designed to gauge employee engagement and satisfaction levels within the company.', 
    'Consider expanding the scope of this survey to include remote employees next year.'),
    
    ('Health & Wellness Survey', 'hr_admin', '2024-08-05', 20, 50, 'Published', 
    'A survey aimed at understanding employee health and wellness priorities, including fitness and mental health.', 
    'The survey should be re-run every quarter to monitor trends in employee wellness.'),
    
    ('Remote Work Feedback', 'manager2', '2024-08-10', 75, 80, 'Published', 
    'This survey collects feedback on the remote work experience, including productivity and work-life balance.', 
    'Follow-up with a deeper dive into specific pain points mentioned in the responses.'),
    
    ('Team Satisfaction Survey', 'manager3', '2024-08-15', 40, 50, 'Published', 
    'An internal survey to assess satisfaction within individual teams, focusing on collaboration and leadership.', 
    'Run this survey again after major team restructuring.'),
    
    ('Company Culture Survey', 'hr_admin', '2024-08-20', 10, 150, 'Draft', 
    'A draft survey for understanding employee perceptions of the company culture, values, and alignment.', 
    'Refine the survey questions before publication to include leadership communication.'),
    
    ('Training Effectiveness', 'trainer1', '2024-08-22', 60, 80, 'Published', 
    'A survey to evaluate the effectiveness of recent training programs and identify areas for improvement.', 
    'Ensure that future surveys focus on feedback from participants with lower scores.'),
    
    ('Leadership Feedback Survey', 'leader1', '2024-08-24', 30, 50, 'Published', 
    'This survey collects anonymous feedback on leadership and management styles within the company.', 
    'Consider breaking down results by department for a more granular view.'),
    
    ('Productivity Check', 'manager4', '2024-08-26', 45, 75, 'Published', 
    'A survey designed to measure employee productivity and efficiency under current work conditions.', 
    'Use this survey to guide future decisions on workload management.'),
    
    ('Client Interaction Survey', 'sales_lead', '2024-08-28', 35, 60, 'Published', 
    'A survey to evaluate employee-client interactions, including communication and relationship-building skills.', 
    'Share results with the client relations team for actionable insights.'),
    
    ('Work-Life Balance Survey', 'hr_admin', '2024-08-30', 25, 100, 'Draft', 
    'This survey assesses how employees are balancing their work and personal lives.', 
    'Consider adding questions about support systems offered by the company.');
    (5, 109, 9),
    (5, 110, 10);

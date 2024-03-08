INSERT INTO customer (name, mobile, email, company, address, description, tags, is_archive)
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

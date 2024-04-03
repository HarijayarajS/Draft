INSERT INTO employee_status_log (created_on, modified_on, employee_id, status, time_taken, is_closed)
VALUES 
('2024-03-25 10:00:00+00', '2024-03-25 10:00:00+00', 1001, 'day-in', 0, false),
('2024-03-25 11:30:00+00', '2024-03-25 11:30:00+00', 1001, 'break-in', 3600, false),
('2024-03-25 13:45:00+00', '2024-03-25 13:45:00+00', 1001, 'lunch-out', 7200, true),
('2024-03-26 09:00:00+00', '2024-03-26 09:00:00+00', 1001, 'day-in', 0, false),
('2024-03-26 10:15:00+00', '2024-03-26 10:15:00+00', 1001, 'break-out', 5400, false),
('2024-03-27 10:30:00+00', '2024-03-27 10:30:00+00', 1001, 'lunch-in', 0, false),
('2024-03-27 11:00:00+00', '2024-03-27 11:00:00+00', 1001, 'day-out', 0, false),
('2024-03-28 09:30:00+00', '2024-03-28 09:30:00+00', 1001, 'break-in', 3600, false),
('2024-03-28 12:00:00+00', '2024-03-28 12:00:00+00', 1001, 'lunch-out', 7200, true),
( '2024-03-29 09:00:00+00', '2024-03-29 09:00:00+00', 1001, 'day-in', 0, false),
( '2024-03-29 11:15:00+00', '2024-03-29 11:15:00+00', 1002, 'break-out', 5400, false),
( '2024-03-30 10:45:00+00', '2024-03-30 10:45:00+00', 1003, 'lunch-in', 0, false),
( '2024-03-30 11:30:00+00', '2024-03-30 11:30:00+00', 1002, 'day-out', 0, false),
( '2024-03-31 09:30:00+00', '2024-03-31 09:30:00+00', 1002, 'break-in', 3600, false),
( '2024-03-31 12:15:00+00', '2024-03-31 12:15:00+00', 1002, 'lunch-out', 7200, true),
( '2024-04-01 09:00:00+00', '2024-04-01 09:00:00+00', 1002, 'day-in', 0, false),
( '2024-04-01 10:45:00+00', '2024-04-01 10:45:00+00', 1002, 'break-out', 5400, false),
( '2024-04-02 11:00:00+00', '2024-04-02 11:00:00+00', 1002, 'lunch-in', 0, false),
( '2024-04-02 11:45:00+00', '2024-04-02 11:45:00+00', 1002, 'day-out', 0, false),
( '2024-04-03 09:15:00+00', '2024-04-03 09:15:00+00', 1002, 'break-in', 3600, false);



INSERT INTO employee_status_log (created_on, modified_on, employee_id, status, time_taken, is_closed)
VALUES 
('2024-03-25 10:00:00+00', '2024-03-25 10:00:00+00', 1001, 'day-in', 0, false),
('2024-03-25 11:30:00+00', '2024-03-25 11:30:00+00', 1002, 'break-in', 3600, false),
('2024-03-25 13:45:00+00', '2024-03-25 13:45:00+00', 1001, 'lunch-out', 7200, true),
('2024-03-26 09:00:00+00', '2024-03-26 09:00:00+00', 1001, 'day-in', 0, false),
('2024-03-26 10:15:00+00', '2024-03-26 10:15:00+00', 1002, 'break-out', 5400, false),
('2024-03-27 10:30:00+00', '2024-03-27 10:30:00+00', 1001, 'lunch-in', 0, false),
('2024-03-27 11:00:00+00', '2024-03-27 11:00:00+00', 1002, 'day-out', 0, false),
('2024-03-28 09:30:00+00', '2024-03-28 09:30:00+00', 1001, 'break-in', 3600, false),
('2024-03-28 12:00:00+00', '2024-03-28 12:00:00+00', 1002, 'lunch-out', 7200, true),
('2024-03-29 09:00:00+00', '2024-03-29 09:00:00+00', 1001, 'day-in', 0, false),
('2024-03-29 11:15:00+00', '2024-03-29 11:15:00+00', 1002, 'break-out', 5400, false),
('2024-03-30 10:45:00+00', '2024-03-30 10:45:00+00', 1001, 'lunch-in', 0, false),
('2024-03-30 11:30:00+00', '2024-03-30 11:30:00+00', 1002, 'day-out', 0, false),
('2024-03-31 09:30:00+00', '2024-03-31 09:30:00+00', 1001, 'break-in', 3600, false),
('2024-03-31 12:15:00+00', '2024-03-31 12:15:00+00', 1002, 'lunch-out', 7200, true),
('2024-04-01 09:00:00+00', '2024-04-01 09:00:00+00', 1001, 'day-in', 0, false),
('2024-04-01 10:45:00+00', '2024-04-01 10:45:00+00', 1002, 'break-out', 5400, false),
('2024-04-02 11:00:00+00', '2024-04-02 11:00:00+00', 1001, 'lunch-in', 0, false),
('2024-04-02 11:45:00+00', '2024-04-02 11:45:00+00', 1002, 'day-out', 0, false),
('2024-04-03 09:15:00+00', '2024-04-03 09:15:00+00', 1001, 'break-in', 3600, false);


SELECT 
                DATE(created_on) AS created_date,
                SUBSTRING(created_on::TEXT, 12, 8) AS in_time,
                SUBSTRING(modified_on::TEXT, 12, 8) AS out_time,
                CAST(SUM(CASE WHEN status = 'break-in' THEN time_taken ELSE 0 END) AS BIGINT) AS break_time, 
                CAST(SUM(CASE WHEN status = 'lunch-in' THEN time_taken ELSE 0 END) AS BIGINT) AS lunch_time 
            FROM 
                employee_status_log 
            WHERE 
                employee_id = $1 
                AND created_on >= $2 
                AND created_on <= $3 
            GROUP BY 
                DATE(created_on)",





SELECT 
    DATE(esl.created_on) AS created_date,
    SUBSTRING(esl.created_on::TEXT, 12, 8) AS in_time,
    SUBSTRING(esl.modified_on::TEXT, 12, 8) AS out_time,
    CAST(SUM(CASE WHEN esl.status = 'break-in' THEN esl.time_taken ELSE 0 END) AS BIGINT) AS break_time, 
    CAST(SUM(CASE WHEN esl.status = 'lunch-in' THEN esl.time_taken ELSE 0 END) AS BIGINT) AS lunch_time 
FROM 
    employee_status_log esl
JOIN
    employees e ON esl.employee_id = e.id
WHERE 
    e.username = 'your_username'
    AND esl.created_on >= $1 
    AND esl.created_on <= $2 
GROUP BY 
    DATE(esl.created_on),
    in_time;


INSERT INTO holiday (date, title) 
VALUES 
    ('2024-01-26', 'Republic Day'),
    ('2024-03-21', 'Holi'),
    ('2024-04-06', 'Mahavir Jayanti'),
    ('2024-04-14', 'Good Friday'),
    ('2024-05-01', 'May Day'),
    ('2024-06-05', 'Eid al-Fitr'),
    ('2024-08-12', 'Eid al-Adha'),
    ('2024-08-15', 'Independence Day'),
    ('2024-10-02', 'Gandhi Jayanti'),
    ('2024-10-25', 'Dussehra'),
    ('2024-11-12', 'Diwali'),
    ('2024-12-25', 'Christmas Day');

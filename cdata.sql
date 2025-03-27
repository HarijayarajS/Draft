

INSERT INTO course_topic (course_id, title, summary, sort_no, category) VALUES  

-- Python Course  
(1, 'Introduction to Python', 'Basic syntax and types', 1, 'Basics'),  
(1, 'Control Flow', 'Conditional statements and loops', 2, 'Fundamentals'),  
(1, 'Functions in Python', 'Defining and using functions', 3, 'Core Concepts'),  
(1, 'Data Structures', 'Lists, Tuples, Sets, and Dictionaries', 4, 'Data Handling'),  
(1, 'Object-Oriented Programming', 'Classes and Objects', 5, 'Advanced Topics'),  
(1, 'File Handling', 'Reading and writing files in Python', 6, 'Data Handling'),  
(1, 'Error Handling', 'Exceptions and debugging techniques', 7, 'Debugging'),  
(1, 'Python Modules & Packages', 'Using built-in and custom modules', 8, 'Core Concepts'),  
(1, 'Python and Databases', 'Using SQLite and PostgreSQL with Python', 9, 'Database'),  
(1, 'Multithreading & Concurrency', 'Working with threads and async programming', 10, 'Advanced Topics'),  
(1, 'Testing in Python', 'Unit testing with pytest and unittest', 11, 'Testing & QA'),  
(1, 'Web Scraping with Python', 'Extracting data from websites', 12, 'Data Science'),  
(1, 'Building REST APIs with Flask', 'Developing APIs using Flask framework', 13, 'Backend Development'),  
(1, 'Django Web Framework', 'Creating web applications with Django', 14, 'Web Development'),  

-- Machine Learning Course  
(2, 'Machine Learning Basics', 'Introduction to ML concepts', 1, 'Machine Learning'),  
(2, 'Supervised Learning', 'Linear regression and classification', 2, 'Machine Learning'),  
(2, 'Unsupervised Learning', 'Clustering and dimensionality reduction', 3, 'Machine Learning'),  
(2, 'Deep Learning Introduction', 'Neural networks and TensorFlow basics', 4, 'Deep Learning'),  
(2, 'Feature Engineering', 'Techniques for improving model performance', 5, 'Machine Learning'),  
(2, 'Hyperparameter Tuning', 'Optimizing ML models with Grid Search & Random Search', 6, 'Machine Learning'),  
(2, 'Natural Language Processing', 'Text processing with NLP techniques', 7, 'AI & NLP'),  
(2, 'Time Series Forecasting', 'Predicting trends with ARIMA and LSTM models', 8, 'Machine Learning'),  
(2, 'Model Deployment', 'Serving ML models using Flask and FastAPI', 9, 'Deployment'),  

-- Data Science Course  
(3, 'Data Analysis', 'Pandas and NumPy basics', 1, 'Data Science'),  
(3, 'Data Cleaning', 'Handling missing values and outliers', 2, 'Data Science'),  
(3, 'Visualization', 'Matplotlib and Seaborn', 3, 'Data Science'),  
(3, 'Exploratory Data Analysis (EDA)', 'Finding patterns in data', 4, 'Data Science'),  
(3, 'Statistical Analysis', 'Hypothesis testing and distributions', 5, 'Statistics'),  
(3, 'Big Data Processing', 'Working with Spark and Hadoop', 6, 'Big Data'),  
(3, 'SQL for Data Science', 'Writing SQL queries for data analysis', 7, 'Database & SQL'),  
(3, 'Data Pipelines', 'Automating data workflows with Apache Airflow', 8, 'Data Engineering'),  
(3, 'Data Science in Production', 'Deploying and monitoring ML models', 9, 'Production Systems'),  

-- Web Development Course  
(4, 'Front-end Development', 'HTML, CSS, JavaScript', 1, 'Web Development'),  
(4, 'JavaScript ES6+', 'Modern JavaScript features', 2, 'Web Development'),  
(4, 'React.js Basics', 'Building interactive UIs with React', 3, 'Frontend Frameworks'),  
(4, 'State Management', 'Managing application state with Redux', 4, 'Frontend Frameworks'),  
(4, 'Backend Development', 'Node.js and Express', 5, 'Web Development'),  
(4, 'REST API Development', 'Building APIs with Express.js', 6, 'Backend Development'),  
(4, 'GraphQL Basics', 'Querying data using GraphQL', 7, 'Backend Development'),  
(4, 'Authentication & Authorization', 'JWT, OAuth, and session handling', 8, 'Security'),  
(4, 'WebSockets & Real-Time Apps', 'Building chat apps with WebSockets', 9, 'Advanced Web'),  
(4, 'Server-Side Rendering (SSR)', 'Optimizing performance with SSR', 10, 'Performance Optimization'),  

-- Cybersecurity Course  
(5, 'Security Basics', 'Network security fundamentals', 1, 'Cybersecurity'),  
(5, 'Ethical Hacking', 'Introduction to ethical hacking', 2, 'Cybersecurity'),  
(5, 'Penetration Testing', 'Finding and exploiting vulnerabilities', 3, 'Cybersecurity'),  
(5, 'Cryptography', 'Encryption techniques and protocols', 4, 'Security'),  
(5, 'Web Security', 'Preventing XSS, CSRF, and SQL injection', 5, 'Web Security'),  
(5, 'Cloud Security', 'Securing AWS, Azure, and GCP environments', 6, 'Cloud Computing'),  
(5, 'Incident Response', 'Handling security breaches', 7, 'Cybersecurity'),  
(5, 'Secure Coding Practices', 'Writing code that prevents attacks', 8, 'Application Security'),  
(5, 'Reverse Engineering', 'Analyzing and understanding malicious code', 9, 'Cybersecurity'),  

-- Mobile Development Course  
(6, 'Android Development', 'Building apps with Kotlin', 1, 'Mobile Development'),  
(6, 'iOS Development', 'Swift programming and Xcode basics', 2, 'Mobile Development'),  
(6, 'React Native', 'Building cross-platform mobile apps', 3, 'Mobile Development'),  
(6, 'Flutter Basics', 'Dart programming and Flutter widgets', 4, 'Mobile Development'),  
(6, 'Push Notifications', 'Implementing FCM and APNs', 5, 'Mobile App Features'),  
(6, 'Mobile Security', 'Protecting mobile apps from vulnerabilities', 6, 'Mobile Security'),  
(6, 'Background Services', 'Running tasks in the background', 7, 'Advanced Mobile Development'),  
(6, 'App Performance Optimization', 'Improving speed and efficiency', 8, 'Mobile Optimization'),  
(6, 'Mobile Payments', 'Integrating Stripe and PayPal in apps', 9, 'Mobile Features'),  
(6, 'AR/VR in Mobile', 'Building augmented and virtual reality apps', 10, 'Advanced Mobile Development');


INSERT INTO faq (id, question, answer, sort_number) VALUES
(1, 'What is this app?', 'This is an AI-driven course app that provides personalized learning experiences.', 1),
(2, 'How does AI enhance my learning?', 'The AI adapts course recommendations, quizzes, and pacing based on your progress.', 2),
(3, 'What types of courses are available?', 'The app offers courses in various fields like programming, business, and design.', 3),
(4, 'How do I enroll in a course?', 'You can browse the catalog, select a course, and start learning instantly.', 4),
(5, 'Can I track my progress?', 'Yes, the app provides insights into your progress and suggests areas for improvement.', 5),
(6, 'Does the app offer certifications?', 'Some courses provide certificates upon completion.', 6),
(7, 'Is there an AI tutor available?', 'Yes, an AI tutor assists with doubts and provides explanations.', 7),
(8, 'Can I customize my learning path?', 'Yes, you can set goals, preferences, and pace to adjust your experience.', 8),
(9, 'Does the app work offline?', 'Some courses can be downloaded for offline learning.', 9),
(10, 'How can I reset my progress?', 'You can reset your progress from settings or contact support.', 10),
(11, 'Is there a mobile version?', 'Yes, the app is available on iOS, Android, and the web.', 11),
(12, 'What should I do if I encounter an issue?', 'Contact support through the app or visit the help center.', 12);
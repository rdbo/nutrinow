/* Use with PostgreSQL */

\c postgres;
DROP DATABASE IF EXISTS nutrinow;
CREATE DATABASE nutrinow;
\c nutrinow;

CREATE TABLE user_account (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(254) UNIQUE NOT NULL,
    gender CHAR(1) NOT NULL,
    birthdate DATE NOT NULL,
    password_hash CHAR(64) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE nutrient (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    unit VARCHAR(100) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE user_nutrition (
    user_id SERIAL,
    nutrient_id SERIAL,
    daily_intake FLOAT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user_account(id),
    FOREIGN KEY (nutrient_id) REFERENCES nutrient(id)
);

/* NOTE: There should be a cron job that will automatically delete expired session tokens */
CREATE TABLE user_session (
    id UUID NOT NULL,
    user_id SERIAL,
    expiry_date DATE NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (user_id) REFERENCES user_account(id)
);

CREATE TABLE food (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE serving (
    id SERIAL,
    food_id SERIAL,
    unit VARCHAR(100) NOT NULL,
    amount FLOAT NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (food_id) REFERENCES food(id)
);

CREATE TABLE serving_nutrient (
    serving_id SERIAL,
    nutrient_id SERIAL,
    amount FLOAT NOT NULL,
    calories FLOAT NOT NULL,
    FOREIGN KEY (serving_id) REFERENCES serving(id),
    FOREIGN KEY (nutrient_id) REFERENCES nutrient(id)
);

CREATE TABLE meal (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE meal_serving (
    meal_id SERIAL,
    serving_id SERIAL,
    FOREIGN KEY (meal_id) REFERENCES meal(id),
    FOREIGN KEY (serving_id) REFERENCES serving(id)
);

CREATE TABLE diet (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE diet_meal (
    diet_id SERIAL,
    meal_id SERIAL,
    FOREIGN KEY (diet_id) REFERENCES diet(id),
    FOREIGN KEY (meal_id) REFERENCES meal(id)
);

INSERT INTO user_account(name, email, gender, birthdate, password_hash) VALUES
    ('Admin', 'admin@localhost', 'M', '1970-01-01', 'f1884986dc7b68fb7ac18c5a7dd2bea6467565300378713fca1fe468bc4be6a6'); /* Password: nutrinow_admin */

INSERT INTO nutrient(name, unit) VALUES
    /* Macronutrients */
    ('Protein', 'g'),
    ('Carbohydrate', 'g'),
    ('Fiber', 'g'),
    ('Saturated Fat', 'g'),
    ('Unsaturated Fat', 'g'),
    ('Trans Fat', 'g'),
    /* Micronutrients */
    ('Vitamin A', 'μg'),
    ('Vitamin B1', 'μg'),
    ('Vitamin B2', 'μg'),
    ('Vitamin B3', 'μg'),
    ('Vitamin B5', 'μg'),
    ('Vitamin B6', 'μg'),
    ('Vitamin B7', 'μg'),
    ('Vitamin B9', 'μg'),
    ('Vitamin B12', 'μg'),
    ('Vitamin C', 'mg'),
    ('Vitamin D', 'μg'),
    ('Vitamin E', 'mg'),
    ('Vitamin K', 'μg'),
    ('Omega-3', 'mg'),
    ('Omega-6', 'g'),
    ('Cholesterol', 'mg'),
    /* Macrominerals */
    ('Calcium', 'mg'),
    ('Magnesium', 'mg'),
    ('Potassium', 'mg'),
    ('Sodium', 'mg'),
    /* Trace Minerals */
    ('Iron', 'mg'),
    ('Iodine', 'μg');


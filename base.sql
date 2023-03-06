/* Use with PostgreSQL */

\c postgres;
DROP DATABASE IF EXISTS nutrinow;
CREATE DATABASE nutrinow;
\c nutrinow;

CREATE TABLE user_account (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    gender CHAR(1) NOT NULL,
    birthdate DATE NOT NULL,
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


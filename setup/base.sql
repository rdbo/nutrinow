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
    weight FLOAT NOT NULL /* in kilograms */,
    birthdate DATE NOT NULL,
    password_hash CHAR(64) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE nutrient (
    id SERIAL,
    name VARCHAR(100) UNIQUE NOT NULL,
    unit VARCHAR(100) NOT NULL,
    PRIMARY KEY(id)
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
    name VARCHAR(255) NOT NULL,
    user_id SERIAL,
    PRIMARY KEY(id),
    FOREIGN KEY (user_id) REFERENCES user_account(id)
);

CREATE TABLE serving (
    id INTEGER, /* SERIAL cannot be NULL */
    food_id SERIAL,
    unit VARCHAR(100) NOT NULL,
    amount FLOAT NOT NULL,
    relative INTEGER NULL, /* contains the ID of another serving, and a relative amount, or NULL. NOTE: Make sure it is never relative to another relative serving */
    PRIMARY KEY(id),
    FOREIGN KEY (food_id) REFERENCES food(id),
    FOREIGN KEY (relative) REFERENCES serving(id)
);

CREATE TABLE serving_nutrient (
    serving_id SERIAL,
    nutrient_id SERIAL,
    amount FLOAT NOT NULL,
    FOREIGN KEY (serving_id) REFERENCES serving(id),
    FOREIGN KEY (nutrient_id) REFERENCES nutrient(id),
    UNIQUE(serving_id, nutrient_id)
);

CREATE TABLE diet (
    id SERIAL,
    name VARCHAR(100) NOT NULL,
    user_id SERIAL,
    PRIMARY KEY(id),
    FOREIGN KEY (user_id) REFERENCES user_account(id)
);

CREATE TABLE diet_nutrition (
    diet_id SERIAL,
    nutrient_id SERIAL,
    min_intake FLOAT,
    max_intake FLOAT,
    relative BOOLEAN NOT NULL, /* relative to body weight */
    FOREIGN KEY (diet_id) REFERENCES diet(id),
    FOREIGN KEY (nutrient_id) REFERENCES nutrient(id),
    UNIQUE(diet_id, nutrient_id)
);

CREATE TABLE meal (
    id SERIAL,
    diet_id SERIAL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (diet_id) REFERENCES diet(id)
);

CREATE TABLE meal_serving (
    id SERIAL,
    meal_id SERIAL,
    serving_id SERIAL,
    amount FLOAT NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (meal_id) REFERENCES meal(id),
    FOREIGN KEY (serving_id) REFERENCES serving(id)
);

CREATE TABLE default_nutrition (
    nutrient_id SERIAL,
    min_intake FLOAT,
    max_intake FLOAT,
    relative BOOLEAN NOT NULL,
    gender CHAR(1) NOT NULL,
    age_min INT NOT NULL,
    age_max INT,
    UNIQUE(nutrient_id, gender, age_min, age_max)
);

/* TODO: Add nutrient categories */

INSERT INTO user_account(name, email, gender, weight, birthdate, password_hash) VALUES
    ('Admin', 'admin@localhost', 'M', '70', '1970-01-01', 'f1884986dc7b68fb7ac18c5a7dd2bea6467565300378713fca1fe468bc4be6a6'); /* Password: nutrinow_admin */

INSERT INTO nutrient(name, unit) VALUES
    /* Macronutrients */
    ('Protein', 'g'),
    ('Carbohydrates', 'g'), /* Total carbohydrates, including fiber and sugar */
    ('Fats', 'g'), /* Total fat, including saturated, unsaturated, and trans fat */
    /* Carbohydrates */
    ('Sugars', 'g'),
    ('Fiber', 'g'),
    /* Lipids (Fats) */
    ('Saturated Fat', 'g'),
    ('Unsaturated Fat', 'g'),
    ('Trans Fat', 'g'),
    /* Vitamins */
    ('Vitamin A', 'μg'),
    ('Vitamin B1', 'mg'),
    ('Vitamin B2', 'mg'),
    ('Vitamin B3', 'mg'),
    ('Vitamin B5', 'mg'),
    ('Vitamin B6', 'mg'),
    ('Vitamin B7', 'μg'),
    ('Vitamin B9', 'μg'),
    ('Vitamin B12', 'μg'),
    ('Vitamin C', 'mg'),
    ('Vitamin D', 'μg'),
    ('Vitamin E', 'mg'),
    ('Vitamin K', 'μg'),
    /* Minerals */
    ('Calcium', 'mg'),
    ('Iron', 'mg'),
    ('Magnesium', 'mg'),
    ('Phosphorus', 'mg'),
    ('Potassium', 'mg'),
    ('Sodium', 'mg'),
    ('Zinc', 'mg'),
    ('Copper', 'mg'),
    ('Manganese', 'mg'),
    ('Selenium', 'mg'),
    /* Others (not really 'nutrients', but they are essential information) */
    ('Water', 'ml'); /* USDA data has water in grams (g), but 1 gram of water = 1 ml of water */
/*    ('Calories', 'kcal');*/ /* calories will be calculated dynamically (Proteins * 4 + Carbos * 4 + Fats * 9) */

/* Default nutrition source: https://www.fda.gov/media/99069/download */

/* Default nutrition >= 4 years old */
INSERT INTO default_nutrition(nutrient_id, min_intake, max_intake, relative, gender, age_min, age_max) VALUES
    ((SELECT id FROM nutrient WHERE name = 'Protein'), 1.0, 2.2, true, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Carbohydrates'), 2.0, 5.0, true, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Fats'), 0.5, 1.5, true, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Sugars'), 0.0, 50.0, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Fiber'), 28.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Saturated Fat'), 0.0, 20.0, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Unsaturated Fat'), NULL, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Trans Fat'), 0.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin A'), 900.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B1'), 1.2, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B2'), 1.3, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B3'), 16.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B5'), 5.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B6'), 1.7, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B7'), 30.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B9'), 400.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B12'), 2.4, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin C'), 90.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin D'), 20.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin E'), 15.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin K'), 120.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Calcium'), 1300.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Iron'), 18.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Magnesium'), 420.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Phosphorus'), 1250.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Potassium'), 4700.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Sodium'), 2300.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Zinc'), 11.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Copper'), 0.9, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Manganese'), 2.3, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Selenium'), 55.0, NULL, false, 'M', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Water'), 35.0, NULL, true, 'M', 4, NULL);

INSERT INTO default_nutrition(nutrient_id, min_intake, max_intake, relative, gender, age_min, age_max) VALUES
    ((SELECT id FROM nutrient WHERE name = 'Protein'), 1.0, 2.2, true, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Carbohydrates'), 2.0, 5.0, true, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Fats'), 0.5, 1.5, true, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Sugars'), 0.0, 50.0, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Fiber'), 28.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Saturated Fat'), 0.0, 20.0, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Unsaturated Fat'), NULL, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Trans Fat'), 0.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin A'), 900.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B1'), 1.2, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B2'), 1.3, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B3'), 16.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B5'), 5.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B6'), 1.7, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B7'), 30.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B9'), 400.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin B12'), 2.4, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin C'), 90.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin D'), 20.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin E'), 15.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Vitamin K'), 120.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Calcium'), 1300.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Iron'), 18.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Magnesium'), 420.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Phosphorus'), 1250.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Potassium'), 4700.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Sodium'), 2300.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Zinc'), 11.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Copper'), 0.9, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Manganese'), 2.3, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Selenium'), 55.0, NULL, false, 'F', 4, NULL),
    ((SELECT id FROM nutrient WHERE name = 'Water'), 35.0, NULL, true, 'F', 4, NULL);

/* TODO: Add default nutrition for age < 4 years old */

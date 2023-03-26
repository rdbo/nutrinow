# Notes:
#  - The default portions for each food is 100g (USDA)
#  - NutriNow must use the same units as USDA (or convert between the units)
#  - If a nutrient is not specified, 0 is assumed (on client-side)

import json

sql_out = open("usda.sql", "w")

usda_file = open("usda.json", "r")
usda_json = json.load(usda_file)
usda_file.close()

# convertion table from USDA nutrient to NutriNow nutrient
usda_cvt = {
    # usda     nutrinow
    ## Macronutrients
    "Protein": "Protein",
    "Carbohydrate, by difference": "Carbohydrates", # Total carbohydrates, including fiber and sugars
    "Total lipid (fat)": "Fats", # Total fats, including satured, unsaturated and trans fats
    ## Carbohydrates
    "Sugars, Total": "Sugars",
    "Fiber, total dietary": "Fiber",
    ## Lipids (Fats)
    "Fatty acids, total saturated": "Saturated Fat",
    "Fatty acids, total monounsaturated": "Unsaturated Fat",
    "Fatty acids, total polyunsaturated": "Unsaturated Fat",
    ### Trans Fat (?)
    ## Vitamins
    "Vitamin A, RAE": "Vitamin A",
    "Thiamin": "Vitamin B1",
    "Riboflavin": "Vitamin B2",
    "Niacin": "Vitamin B3",
    "Pantothenic acid" : "Vitamin B5",
    "Vitamin B-6": "Vitamin B6",
    "Biotin": "Vitamin B7",
    "Folate, total": "Vitamin B9",
    "Vitamin B-12": "Vitamin B12",
    "Vitamin C, total ascorbic acid": "Vitamin C",
    "Vitamin D (D2 + D3)": "Vitamin D",
    "Vitamin E (alpha-tocopherol)": "Vitamin E",
    "Vitamin K (phylloquinone)": "Vitamin K",
    "Vitamin K (Dihydrophylloquinone)": "Vitamin K",
    "Vitamin K (Menaquinone-4)": "Vitamin K",
    ## Minerals
    "Calcium, Ca": "Calcium",
    "Iron, Fe": "Iron",
    "Magnesium, Mg": "Magnesium",
    "Phosphorus, P": "Phosphorus",
    "Potassium, K": "Potassium",
    "Sodium, Na": "Sodium",
    "Zinc, Zn": "Zinc",
    "Copper, Cu": "Copper",
    "Manganese, Mn": "Manganese",
    "Selenium, Se": "Selenium",
    ## Others
    "Water": "Water", # note: water is in grams on USDA data, and in mililiters on NutriNow, but 1g of water = 1ml of water
    ### the foods have either 'Energy' or 'Energy (Atwater...)', never both (it seems)
    ### "Energy": "Calories",
    ### "Energy (Atwater General Factors)": "Calories"
    ### note: calories will be calculated on client-side
}

# starting IDs
next_food_id = "(COALESCE((SELECT SUM((SELECT id FROM food ORDER BY id DESC LIMIT 1) + 1)), 1))"
food_id = "(SELECT id FROM food ORDER BY id DESC LIMIT 1)"
next_serving_id = "(COALESCE((SELECT SUM((SELECT id FROM serving ORDER BY id DESC LIMIT 1) + 1)), 1))"
serving_id = "(SELECT id FROM serving ORDER BY id DESC LIMIT 1)"

dataset_name = list(usda_json)[0]

for obj in usda_json[dataset_name]:
    nutrients = {}
    food_name = obj["description"].replace("'", "''")
    sql_out.write(f"INSERT INTO food(id, name, user_id) VALUES ({next_food_id}, '{food_name}', 1);\n")
    sql_out.write(f"INSERT INTO serving(id, food_id, unit, amount) VALUES({next_serving_id}, {food_id}, 'g', 100);\n")
    for food_nutrient in obj["foodNutrients"]:
        # ignore entries that are not nutrients
        if food_nutrient["type"] != "FoodNutrient":
            continue
        # ignore nutrients that are not in the database
        if food_nutrient["nutrient"]["name"] not in usda_cvt:
            continue

        nutrient_name = usda_cvt[food_nutrient["nutrient"]["name"]]
        nutrient_amount = food_nutrient["amount"]

        if nutrient_name in nutrients:
            # account for summed nutrients (e.g unsatured fats)
            nutrients[nutrient_name]["amount"] += nutrient_amount
        else:
            nutrients[nutrient_name] = { "amount": nutrient_amount, "unit": food_nutrient["nutrient"]["unitName"] }
    
    for nutrient in nutrients:
        nutrient_id = f"(SELECT id FROM nutrient WHERE name = '{nutrient}')"
        nutrient_amount = nutrients[nutrient]["amount"]
        sql_out.write(f"INSERT INTO serving_nutrient(serving_id, nutrient_id, amount) VALUES({serving_id}, {nutrient_id}, {nutrient_amount});\n")

    sql_out.write("\n")
    # TODO: Add servings provided by USDA (foodPortions)

sql_out.close()

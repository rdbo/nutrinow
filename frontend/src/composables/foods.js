export function getServingNutrients(servings, index) {
    let serving = servings[index];
    if (!serving.relative)
        return serving.nutrients;

    for (let i = 0; i < servings.length; ++i) {
        let base_serving = servings[i];
        if (base_serving.id == serving.relative) {
            let nutrients = base_serving.nutrients;
            for (let j = 0; j < nutrients.length; ++j) {
                nutrients[j].amount *= serving.amount / base_serving.amount; // calculate relative nutrient amount
            }
            return nutrients;
        }
    }

    return [];
}

export function getNutrientsFromMealFood(food) {
    let nutrient_ratio = food.serving_amount / food.serving_base; // proportionally calculate nutrient amount from serving base amount
    let nutrient_list = [];
    for (let i = 0; i < food.base_nutrients.length; ++i) {
        let nutrient = food.base_nutrients[i];
        nutrient_list.push({
            name: nutrient.name,
            amount: Number(nutrient.amount * nutrient_ratio).toFixed(1),
            unit: nutrient.unit
        });
    }

    return nutrient_list;
}

export function getDisplayNutrients(nutrients) {
    let displayNutrients = {
        "Protein": { amount: 0, unit: ""},
        "Carbohydrates": { amount: 0, unit: "" },
        "Fats": { amount: 0, unit: "" },
        "Calories": { amount: 0, unit: "" }
    };
    for (let i = 0; i < nutrients.length; ++i) {
        if (displayNutrients.hasOwnProperty(nutrients[i].name)) {
            displayNutrients[nutrients[i].name].amount = Number(nutrients[i].amount).toFixed(1);
            displayNutrients[nutrients[i].name].unit = nutrients[i].unit;
        }
    }

    // calculate calories based on other macronutrients
    let calories = Number(displayNutrients["Protein"].amount * 4 + displayNutrients["Carbohydrates"].amount * 4 + displayNutrients["Fats"].amount * 9).toFixed(1);
    if (calories > 0) {
        displayNutrients["Calories"].amount = calories;
        displayNutrients["Calories"].unit = "kcal";
    }

    let displayNutrientList = [];
    for (let nutrient in displayNutrients) {
        displayNutrientList.push({
            name: nutrient,
            amount: displayNutrients[nutrient].amount,
            unit: displayNutrients[nutrient].unit
        });
    }

    return displayNutrientList;
}

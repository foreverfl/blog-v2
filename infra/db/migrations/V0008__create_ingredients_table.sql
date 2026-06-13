-- V8: Create recipe.ingredients table + seed common ingredients

CREATE TABLE IF NOT EXISTS recipe.ingredients (
    id       uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    slug     text UNIQUE NOT NULL
             CHECK (slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$'),   -- lowercase/digits, hyphen-separated only (blocks caps/space/underscore)
    name_ko  text NOT NULL,
    name_ja  text NOT NULL,
    name_en  text NOT NULL,
    category text                    -- seasoning/oil/spice/herb/vegetable/meat/seafood/dairy/grain
);

-- Seed: common cooking ingredients (idempotent on slug)
INSERT INTO recipe.ingredients (slug, name_ko, name_ja, name_en, category) VALUES
    -- seasoning
    ('salt','소금','塩','Salt','seasoning'),
    ('sugar','설탕','砂糖','Sugar','seasoning'),
    ('soy-sauce','간장','醤油','Soy sauce','seasoning'),
    ('fish-sauce','액젓','魚醤','Fish sauce','seasoning'),
    ('oyster-sauce','굴소스','オイスターソース','Oyster sauce','seasoning'),
    ('vinegar','식초','酢','Vinegar','seasoning'),
    ('mirin','미림','みりん','Mirin','seasoning'),
    ('cooking-wine','맛술','料理酒','Cooking wine','seasoning'),
    ('miso','미소','味噌','Miso','seasoning'),
    ('gochujang','고추장','コチュジャン','Gochujang','seasoning'),
    ('doenjang','된장','テンジャン','Doenjang','seasoning'),
    ('ketchup','케첩','ケチャップ','Ketchup','seasoning'),
    ('mayonnaise','마요네즈','マヨネーズ','Mayonnaise','seasoning'),
    ('mustard','머스터드','マスタード','Mustard','seasoning'),
    -- oil
    ('sesame-oil','참기름','ごま油','Sesame oil','oil'),
    ('olive-oil','올리브유','オリーブオイル','Olive oil','oil'),
    ('vegetable-oil','식용유','サラダ油','Vegetable oil','oil'),
    -- spice
    ('black-pepper','후추','黒胡椒','Black pepper','spice'),
    ('red-pepper-powder','고춧가루','唐辛子粉','Red pepper powder','spice'),
    ('garlic-powder','마늘가루','ガーリックパウダー','Garlic powder','spice'),
    ('curry-powder','카레가루','カレー粉','Curry powder','spice'),
    ('cumin','큐민','クミン','Cumin','spice'),
    ('paprika','파프리카가루','パプリカ','Paprika','spice'),
    -- herb
    ('basil','바질','バジル','Basil','herb'),
    ('parsley','파슬리','パセリ','Parsley','herb'),
    ('cilantro','고수','パクチー','Cilantro','herb'),
    ('rosemary','로즈마리','ローズマリー','Rosemary','herb'),
    ('thyme','타임','タイム','Thyme','herb'),
    -- vegetable
    ('tofu','두부','豆腐','Tofu','vegetable'),
    ('garlic','마늘','にんにく','Garlic','vegetable'),
    ('onion','양파','玉ねぎ','Onion','vegetable'),
    ('green-onion','대파','長ねぎ','Green onion','vegetable'),
    ('ginger','생강','生姜','Ginger','vegetable'),
    ('carrot','당근','にんじん','Carrot','vegetable'),
    ('potato','감자','じゃがいも','Potato','vegetable'),
    ('tomato','토마토','トマト','Tomato','vegetable'),
    ('chili-pepper','고추','唐辛子','Chili pepper','vegetable'),
    ('cabbage','양배추','キャベツ','Cabbage','vegetable'),
    ('spinach','시금치','ほうれん草','Spinach','vegetable'),
    ('mushroom','버섯','きのこ','Mushroom','vegetable'),
    -- meat
    ('pork','돼지고기','豚肉','Pork','meat'),
    ('beef','소고기','牛肉','Beef','meat'),
    ('chicken','닭고기','鶏肉','Chicken','meat'),
    ('egg','계란','卵','Egg','meat'),
    -- seafood
    ('shrimp','새우','エビ','Shrimp','seafood'),
    ('squid','오징어','イカ','Squid','seafood'),
    ('anchovy','멸치','煮干し','Anchovy','seafood'),
    ('clam','조개','あさり','Clam','seafood'),
    -- dairy
    ('butter','버터','バター','Butter','dairy'),
    ('milk','우유','牛乳','Milk','dairy'),
    ('cheese','치즈','チーズ','Cheese','dairy'),
    ('cream','생크림','生クリーム','Cream','dairy'),
    -- grain
    ('rice','쌀','米','Rice','grain'),
    ('flour','밀가루','小麦粉','Flour','grain'),
    ('noodles','면','麺','Noodles','grain')
ON CONFLICT (slug) DO NOTHING;

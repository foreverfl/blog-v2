-- V11: Create diet_dishes + diet_meals — the intake side of calorie tracking, with a dish seed

-- Shared dictionary of frequently eaten dishes. The registrant writes `name` in
-- their own language; name_ko/en/ja are filled later by a background batch.
-- kcal is per one serving as named — diet_meals.quantity multiplies it.
CREATE TABLE IF NOT EXISTS public.diet_dishes (
    id              uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    slug            text UNIQUE NOT NULL
                    CHECK (slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$'),
    name            text NOT NULL,
    name_ko         text,                          -- NULL until the batch translates it
    name_en         text,
    name_ja         text,
    kcal            int  NOT NULL CHECK (kcal > 0),
    main_ingredient text,                          -- recipe.ingredients slug, for "what can I make with tofu"
    note            text
);

-- One eaten item = one row. Grouped by date only, with no FK to diet_daily_logs,
-- so meals can be written before the day's activity record exists.
CREATE TABLE IF NOT EXISTS public.diet_meals (
    id         uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    uuid NOT NULL,
    log_date   date NOT NULL,
    dish_id    uuid,                                -- NULL when typed in freely instead of picked
    name       text NOT NULL,
    kcal       int  NOT NULL CHECK (kcal >= 0),
    quantity   numeric(4,1) NOT NULL DEFAULT 1,
    created_at timestamptz DEFAULT now() NOT NULL,
    CONSTRAINT diet_meals_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE,
    -- name and kcal are already copied into this row, so the record survives the dish going away.
    CONSTRAINT diet_meals_dish_id_fkey FOREIGN KEY (dish_id) REFERENCES public.diet_dishes(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_diet_meals_user_date ON public.diet_meals (user_id, log_date);

-- Seed: dishes that recur in the author's own log (idempotent on slug).
-- kcal are representative estimates, not measured values.
INSERT INTO public.diet_dishes (slug, name, kcal, main_ingredient, note) VALUES
    ('gochujang-jjigae',       '고추장찌개 1그릇',     300, 'pork',    '큰 그릇은 1.8그릇 정도'),
    ('kimchi-jjigae',          '김치찌개 1그릇',       250, 'pork',    NULL),
    ('haemul-doenjang-jjigae', '해물된장찌개',         270, 'tofu',    NULL),
    ('samgyeopsal-100g',       '삼겹살 구이 100g',     360, 'pork',    NULL),
    ('chadol-100g',            '차돌박이 100g',        350, 'beef',    NULL),
    ('tofu-block',             '두부 1모',             220, 'tofu',    '木綿 300g 기준. 絹이면 170, 구우면 기름만큼 추가'),
    ('boiled-egg',             '삶은 계란 1개',         70, 'egg',     NULL),
    ('karaage-piece',          '가라아게 1조각',        70, 'chicken', NULL),
    ('yurinki',                '유린기 닭다리 1장',    600, 'chicken', '250~300g. 튀기면 700, 껍질 빼면 450'),
    ('baguette-slice',         '바게트 1조각',          85, 'flour',   NULL),
    ('cup-noodles',            'カップヌードル 1개',   330, 'noodles', NULL),
    ('soboro-don-topping',     '소보로동 토핑(밥 없이)', 300, 'egg',   '계란 1개 + 간고기 70~100g'),
    ('highball-45ml',          '하이볼 1잔(위스키 45ml)', 100, NULL,   '위스키 40도 기준. 지거 반대편 30ml면 65')
ON CONFLICT (slug) DO NOTHING;

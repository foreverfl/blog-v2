-- V10: Create diet_tdee_cases + diet_daily_logs — the burn side of calorie tracking

-- Per-user "total daily burn" presets. No seed: everyone registers their own
-- values from the UI, since the numbers differ per person.
CREATE TABLE IF NOT EXISTS public.diet_tdee_cases (
    id      uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    name    text NOT NULL,                  -- free-form, any language: 'wfh', '出社'
    kcal    int  NOT NULL CHECK (kcal > 0),
    CONSTRAINT diet_tdee_cases_user_id_name_key UNIQUE (user_id, name),
    CONSTRAINT diet_tdee_cases_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);

-- One row per user per day.
CREATE TABLE IF NOT EXISTS public.diet_daily_logs (
    user_id      uuid NOT NULL,
    log_date     date NOT NULL,
    tdee_case_id uuid NOT NULL,
    walk_minutes int  NOT NULL DEFAULT 0 CHECK (walk_minutes >= 0),
    weight_kg    numeric(4,1),
    note         text,
    CONSTRAINT diet_daily_logs_pkey PRIMARY KEY (user_id, log_date),
    CONSTRAINT diet_daily_logs_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE,
    -- Not RESTRICT: NO ACTION defers the check to end of statement, so deleting a user still cascades.
    CONSTRAINT diet_daily_logs_tdee_case_id_fkey FOREIGN KEY (tdee_case_id) REFERENCES public.diet_tdee_cases(id)
);

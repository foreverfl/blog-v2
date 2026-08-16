-- V9: Create diet_profiles table — one body profile per user (1:1 with users)

-- Stores only the inputs of the derived metrics: BMI, required deficit and the
-- walking equivalent are computed on read, never stored.
-- The daily-changing weight lives in diet_daily_logs.
CREATE TABLE IF NOT EXISTS public.diet_profiles (
    user_id          uuid PRIMARY KEY,                            -- PK = FK enforces 1:1
    height_cm        numeric(4,1) NOT NULL CHECK (height_cm > 0), -- 176.0
    target_weight_kg numeric(4,1),                                -- 65.0
    bmr_kcal         int CHECK (bmr_kcal > 0),                    -- 1700, entered by hand
    updated_at       timestamptz DEFAULT now() NOT NULL,
    CONSTRAINT diet_profiles_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);

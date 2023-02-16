-- Modify table `users` adding a new column `is_admin` of type `boolean` with default value `false` and not null constraint

ALTER TABLE users ADD COLUMN IF NOT EXISTS is_admin boolean NOT NULL DEFAULT false;
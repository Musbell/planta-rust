CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "User" (
                        "id" UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                        "createdAt" timestamp(3) NOT NULL DEFAULT current_timestamp,
                        "updatedAt" timestamp(3) NOT NULL DEFAULT current_timestamp,
                        "firstName" VARCHAR(191) NOT NULL,
                        "lastName" VARCHAR(191) NOT NULL,
                        "email" VARCHAR(191),
                        "middleName" VARCHAR(191),
                        UNIQUE ("email")
);

CREATE INDEX idx_user_lastName ON "User" ("lastName");
CREATE INDEX idx_user_firstName ON "User" ("firstName");
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

CREATE TABLE "Profile" (
                           "id" UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                           "createdAt" timestamp(3) NOT NULL DEFAULT current_timestamp,
                           "updatedAt" timestamp(3) NOT NULL DEFAULT current_timestamp,
                           "bio" VARCHAR(191),
                           "accountNumber" VARCHAR(191),
                           "bvn" VARCHAR(191),
                           "gender" VARCHAR(191),
                           "identityNumber" VARCHAR(191),
                           "nationality" VARCHAR(191),
                           "phoneNumber" VARCHAR(191),
                           "userId" UUID NOT NULL,
                           UNIQUE ("userId"),
                           FOREIGN KEY ("userId") REFERENCES "User" ("id")
);

CREATE INDEX idx_profile_userId ON "Profile" ("userId");
CREATE INDEX idx_profile_gender ON "Profile" ("gender");
CREATE INDEX idx_profile_nationality ON "Profile" ("nationality");

CREATE TABLE "Farm" (
                        "id" UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                        "createdAt" timestamp(3) NOT NULL DEFAULT current_timestamp,
                        "updatedAt" timestamp(3) NOT NULL DEFAULT current_timestamp,
                        "farm_name" VARCHAR(191),
                        "acreage" DOUBLE PRECISION NOT NULL,
                        "state" VARCHAR(191),
                        "locality" VARCHAR(191),
                        "has_drainage_tile" BOOLEAN,
                        "land_value" INT,
                        "is_irrigated" BOOLEAN,
                        "ownership" VARCHAR(6) CHECK (ownership IN ('RENT', 'OWNER', 'INHERIT')) DEFAULT 'RENT',
                        "available_portion" DOUBLE PRECISION,
                        "country" VARCHAR(191),
                        "farmerId" UUID NOT NULL,
                        "latitude" DOUBLE PRECISION,
                        "longitude" DOUBLE PRECISION,
                        "farm_site" VARCHAR(191),
                        FOREIGN KEY ("farmerId") REFERENCES "User" ("id")
);

CREATE INDEX idx_farm_farmerId ON "Farm" ("farmerId");
CREATE INDEX idx_farm_state ON "Farm" ("state");
CREATE INDEX idx_farm_locality ON "Farm" ("locality");
CREATE INDEX idx_farm_country ON "Farm" ("country");

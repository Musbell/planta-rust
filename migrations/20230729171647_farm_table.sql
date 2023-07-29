CREATE TABLE "Farm" (
                        "id" TEXT PRIMARY KEY,
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
                        "farmerId" TEXT NOT NULL,
                        "latitude" DOUBLE PRECISION,
                        "longitude" DOUBLE PRECISION,
                        "farm_site" VARCHAR(191),
                        FOREIGN KEY ("farmerId") REFERENCES "User" ("id")
);

CREATE INDEX idx_farm_farmerId ON "Farm" ("farmerId");
CREATE INDEX idx_farm_state ON "Farm" ("state");
CREATE INDEX idx_farm_locality ON "Farm" ("locality");
CREATE INDEX idx_farm_country ON "Farm" ("country");
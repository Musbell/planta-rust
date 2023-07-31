CREATE TABLE "Profile" (
                           "id" UUID PRIMARY KEY,
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

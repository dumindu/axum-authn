CREATE TABLE "users" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    "id" UUID NOT NULL,
    "is_verified" BOOLEAN NOT NULL,
    "email" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_users_by_email" ON "users" ("email");

CREATE TABLE "user_passwords" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    "user_id" UUID NOT NULL,
    "password_hash" TEXT NOT NULL,
    PRIMARY KEY ("user_id")
);

CREATE TABLE "user_identities" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "provider" TEXT NOT NULL,
    "provider_sub_id" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_user_identities_by_provider_and_provider_sub_id" ON "user_identities" ("provider", "provider_sub_id");
CREATE INDEX "index_user_identities_by_user_id" ON "user_identities" ("user_id");

CREATE TABLE "user_devices" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "last_used_at" TIMESTAMPTZ(6) NOT NULL,
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "signature_counter" SMALLINT NOT NULL,
    "device_name" TEXT NOT NULL,
    "device_hash" BYTEA NOT NULL,
    "credential_id" BYTEA NOT NULL,
    "public_key" BYTEA NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_user_devices_by_user_id_and_device_hash" ON "user_devices" ("user_id", "device_hash");
CREATE UNIQUE INDEX "index_user_devices_by_credential_id" ON "user_devices" ("credential_id");

CREATE TABLE "refresh_tokens" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "device_id" UUID NOT NULL,
    "token_family_id" UUID NOT NULL,
    "is_revoked" BOOLEAN NOT NULL,
    "token_hash" BYTEA NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_refresh_tokens_by_token_hash" ON "refresh_tokens" ("token_hash");
CREATE INDEX "index_refresh_tokens_by_token_family_id" ON "refresh_tokens" ("token_family_id");

CREATE TABLE "oauth_challenges" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "state" UUID NOT NULL,
    "pkce_code_verifier" TEXT NOT NULL,
    "client_redirect_uri" TEXT NOT NULL,
    "provider" TEXT NOT NULL,
    "flow_type" TEXT NOT NULL,
    PRIMARY KEY ("state")
);

CREATE TABLE "webauthn_challenges" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "user_id" UUID,
    "device_id" UUID,
    "challenge_token" BYTEA NOT NULL,
    "challenge_type" TEXT NOT NULL,
    PRIMARY KEY ("challenge_token")
);
CREATE INDEX "index_webauthn_challenges_by_expires_at" ON "webauthn_challenges" ("expires_at");

CREATE TABLE "security_audit_logs" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "id" UUID NOT NULL,
    "user_id" UUID,
    "device_id" UUID,
    "ip_address" TEXT NOT NULL,
    "user_agent" TEXT NOT NULL,
    "location" TEXT,
    "event_type" TEXT NOT NULL,
    "context" TEXT,
    PRIMARY KEY ("id")
);
CREATE INDEX "index_security_audit_logs_by_user_id_and_device_id" ON "security_audit_logs" ("user_id", "device_id");

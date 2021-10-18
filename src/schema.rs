table! {
    customers (id) {
        id -> Uuid,
        stripe_customer_id -> Nullable<Text>,
    }
}
// Random comment I'm not going to like myself later
table! {
    membean (question) {
        question -> Varchar,
        answer -> Varchar,
    }
}

table! {
    prices (id) {
        id -> Text,
        product_id -> Nullable<Text>,
        active -> Nullable<Bool>,
        description -> Nullable<Text>,
        unit_amount -> Nullable<Int8>,
        currency -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Pricing_type>,
        interval -> Nullable<Pricing_plan_interval>,
        interval_count -> Nullable<Int4>,
        trial_period_days -> Nullable<Int4>,
        metadata -> Nullable<Jsonb>,
    }
}

table! {
    products (id) {
        id -> Text,
        active -> Nullable<Bool>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
    }
}

table! {
    subscriptions (id) {
        id -> Text,
        user_id -> Uuid,
        status -> Nullable<Subscription_status>,
        metadata -> Nullable<Jsonb>,
        price_id -> Nullable<Text>,
        quantity -> Nullable<Int4>,
        cancel_at_period_end -> Nullable<Bool>,
        created -> Timestamptz,
        current_period_start -> Timestamptz,
        current_period_end -> Timestamptz,
        ended_at -> Nullable<Timestamptz>,
        cancel_at -> Nullable<Timestamptz>,
        canceled_at -> Nullable<Timestamptz>,
        trial_start -> Nullable<Timestamptz>,
        trial_end -> Nullable<Timestamptz>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        full_name -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        billing_address -> Nullable<Jsonb>,
        payment_method -> Nullable<Jsonb>,
    }
}

joinable!(prices -> products (product_id));
joinable!(subscriptions -> prices (price_id));

allow_tables_to_appear_in_same_query!(
    customers,
    membean,
    prices,
    products,
    subscriptions,
    users,
);

Product
    name Spont(title)
    description Spont(info)
    pos_display_name (new)
    kitchen_display_name (new)
    colour
    images images[]
    tags String[]
    linked_attributes LinkedAttributes[]
    calorie_count (new) small int
    can_ask_quantity (new) bool
    has_alcohol (new) bool
    dietary_options (new) String[]
    ingredients (item contains) String[]
    allergies ProductAllergy[]
    barcodes String[]
    cost_price float
    day_stock ProductDayStock
    extra1
    extra2 Option(String)
    extra3 Option(String)
    image
    is_archive bool
    is_dial_pad bool
    is_dynamic bool
    is_finance bool
    is_hide_in_production bool
    is_in_stock bool
    is_inventory_comb bool
    is_show_detail bool
    ledger
    price_excl float
    price_incl float
    price_tier Option(ProductPriceTier)
    production_note
    production_title
    related_ids
    related_info ProductRelatedInfo[]
    related Option(String)
    report_category_id
    sort_map (new) Map(String, i16)
    special_allergies String[]
    stock small int
    supplier
    unit_base
    unit_system
    user_tags String[]
    vat small int

images
    path
    name
    is_thumb
    size

LinkedAttributes
    id
    is_active

ProductPriceTier
    id
    price_excl
    price_incl
    quantity

ProductRelatedInfo
    id
    info
    product_ids
    type
    title

ProductDayStock
    attribute_id
    hidden_options
    is_hidden

ProductAllergy
    identifier
    is_selected
    label

# Product Structure (Grouped)

## Basic Information
- name *(Spont(title))*
- description *(Spont(info))*
- pos_display_name *(new)*
- kitchen_display_name *(new)*
- colour

## Media
- image
- images *(images[])*

## Tags & Classification
- tags *(String[])*
- user_tags *(String[])*
- report_category_id
- sort_map *(Map(String, i16), new)*

## Attributes
- linked_attributes *(LinkedAttributes[])*
- related_ids
- related *(Option<String>)*
- related_info *(ProductRelatedInfo[])*

## Food / Health / Dietary
- calorie_count *(small int, new)*
- dietary_options *(String[], new)*
- ingredients *(String[])*
- allergies *(ProductAllergy[])*
- special_allergies *(String[])*
- has_alcohol *(bool, new)*

## Pricing
- cost_price *(float)*
- price_excl *(float)*
- price_incl *(float)*
- price_tier *(Option<ProductPriceTier>)*
- vat *(small int)*

## Inventory / Stock
- stock *(small int)*
- day_stock *(ProductDayStock)*
- is_in_stock
- is_inventory_comb
- can_ask_quantity *(bool, new)*
- barcodes *(String[])*

## Production
- production_title
- production_note
- is_hide_in_production

## Flags / Behaviour
- is_archive
- is_dial_pad
- is_dynamic
- is_finance
- is_show_detail

## Units
- unit_base
- unit_system

## Vendor / Accounts
- supplier
- ledger

## Misc
- extra1
- extra2 *(Option<String>)*
- extra3 *(Option<String>)*

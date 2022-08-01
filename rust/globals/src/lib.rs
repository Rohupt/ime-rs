use windows::core::GUID;

//---------------------------------------------------------------------
// SampleIME CLSID
//---------------------------------------------------------------------
// {89adf27c-0863-4ad3-a463-f7ecfc75d01d}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_CLSID: GUID = GUID::from_u128(0x89adf27c_0863_4ad3_a463_f7ecfc75d01d);

//---------------------------------------------------------------------
// Profile GUID
//---------------------------------------------------------------------
// {2b574d86-3844-4136-8dd9-41e7829ac9f8}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_PROFILE: GUID = GUID::from_u128(0x2b574d86_3844_4136_8dd9_41e7829ac9f8);

//---------------------------------------------------------------------
// PreserveKey GUID
//---------------------------------------------------------------------
// {27579631-4620-4640-b3e0-ae24541c216c}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_IME_MODE_PRESERVE_KEY: GUID = GUID::from_u128(0x27579631_4620_4640_b3e0_ae24541c216c);

// {8b5255fb-0ca8-431b-a54b-b06a060c73f4}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_DOUBLE_SINGLE_BYTE_PRESERVE_KEY: GUID = GUID::from_u128(0x8b5255fb_0ca8_431b_a54b_b06a060c73f4);

// {04250a07-9b7b-4b9d-b7a9-8dc3a62c71d1}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_PUNCTUATION_PRESERVE_KEY: GUID = GUID::from_u128(0x04250a07_9b7b_4b9d_b7a9_8dc3a62c71d1);// {8b5255fb-0ca8-431b-a54b-b06a060c73f4}

//---------------------------------------------------------------------
// Compartments
//---------------------------------------------------------------------
// {88677281-8ae0-4b88-bb6b-a36101cc0eea}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_COMPARTMENT_DOUBLE_SINGLE_BYTE: GUID = GUID::from_u128(0x88677281_8ae0_4b88_bb6b_a36101cc0eea);

// {0a83989c-1b64-4374-ae3d-e60b7208b36f}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_COMPARTMENT_PUNCTUATION: GUID = GUID::from_u128(0x0a83989c_1b64_4374_ae3d_e60b7208b36f);

//---------------------------------------------------------------------
// LanguageBars
//---------------------------------------------------------------------

// {9826a3f7-a822-43f7-a51b-fbe7b55995d3}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_DISPLAY_ATTRIBUTE_INPUT: GUID = GUID::from_u128(0x9826a3f7_a822_43f7_a51b_fbe7b55995d3);

// {36a97adf-e994-4164-b271-5f169689093e}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_DISPLAY_ATTRIBUTE_CONVERTED: GUID = GUID::from_u128(0x36a97adf_e994_4164_b271_5f169689093e);

//---------------------------------------------------------------------
// UI element
//---------------------------------------------------------------------

// {c2bc76f1-9b5b-4a88-9620-8ce35d368457}
#[no_mangle]
// #[cfg(target_arch = "x86")]
pub static SAMPLEIME_GUID_CAND_UIELEMENT: GUID = GUID::from_u128(0xc2bc76f1_9b5b_4a88_9620_8ce35d368457);

// --------------------------------------------------------------------
// For x64 version if needed
// --------------------------------------------------------------------

// // {bda8df31-143a-464b-accd-523622072467}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_CLSID: GUID = GUID::from_u128(0xbda8df31_143a_464b_accd_523622072467);

// // {3499af8d-159b-41b0-a3e1-54e055e187fd}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_PROFILE: GUID = GUID::from_u128(0x3499af8d_159b_41b0_a3e1_54e055e187fd);

// // {7a5478c0-5e90-40d5-8dcf-917938b712d6}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_IME_MODE_PRESERVE_KEY: GUID = GUID::from_u128(0x7a5478c0_5e90_40d5_8dcf_917938b712d6);

// // {361d6a74-f5db-490d-8247-146e3a9e7b84}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_DOUBLE_SINGLE_BYTE_PRESERVE_KEY: GUID = GUID::from_u128(0x361d6a74_f5db_490d_8247_146e3a9e7b84);

// // {2e5470f9-9df5-4526-845d-b512307bf82a}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_PUNCTUATION_PRESERVE_KEY: GUID = GUID::from_u128(0x2e5470f9_9df5_4526_845d_b512307bf82a);

// // {bc876c25-e580-457d-a8a7-995e56f9f082}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_COMPARTMENT_DOUBLE_SINGLE_BYTE: GUID = GUID::from_u128(0xbc876c25_e580_457d_a8a7_995e56f9f082);

// // {9764f9a4-168f-4333-8ffa-ba946b144560}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_COMPARTMENT_PUNCTUATION: GUID = GUID::from_u128(0x9764f9a4_168f_4333_8ffa_ba946b144560);

// // {68812349-6987-4ad6-a5ef-1d0fa97935db}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_DISPLAY_ATTRIBUTE_INPUT: GUID = GUID::from_u128(0x68812349_6987_4ad6_a5ef_1d0fa97935db);

// // {7907d79f-c026-444b-b93a-dfeae2778a53}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_DISPLAY_ATTRIBUTE_CONVERTED: GUID = GUID::from_u128(0x7907d79f_c026_444b_b93a_dfeae2778a53);

// // {f26cf555-7e28-4e7f-a13a-017c288df0ce}
// #[no_mangle]
// #[cfg(target_arch = "x86_64")]
// pub static SAMPLEIME_GUID_CAND_UIELEMENT: GUID = GUID::from_u128(0xf26cf555_7e28_4e7f_a13a_017c288df0ce);
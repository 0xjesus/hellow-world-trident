// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod hello_world_instruction {
    use trident_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        65u8, 63u8, 75u8, 161u8, 197u8, 207u8, 132u8, 163u8, 196u8, 255u8, 157u8, 255u8, 11u8,
        141u8, 97u8, 175u8, 98u8, 103u8, 111u8, 15u8, 249u8, 1u8, 228u8, 63u8, 2u8, 168u8, 170u8,
        97u8, 151u8, 212u8, 199u8, 224u8,
    ]);
    pub async fn initialize(
        client: &Client,
        i_input: u8,
        a_author: Pubkey,
        a_hello_world_account: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                hello_world::instruction::Initialize { input: i_input },
                hello_world::accounts::Initialize {
                    author: a_author,
                    hello_world_account: a_hello_world_account,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn initialize_ix(
        i_input: u8,
        a_author: Pubkey,
        a_hello_world_account: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: hello_world::instruction::Initialize { input: i_input }.data(),
            accounts: hello_world::accounts::Initialize {
                author: a_author,
                hello_world_account: a_hello_world_account,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}

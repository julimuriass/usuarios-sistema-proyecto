#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod usuarios_sistema {
    use ink::prelude::{format, string::String};
    use ink::storage::Mapping;    

    #[ink(storage)]

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    pub struct Sistema {
        value: bool, //Tendríamos que dejar el value??
        usuarios: ink::storage::Mapping<AccountId, Usuario>,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]

    pub enum ErrorSistema {
        UsuarioYaRegistrado,
        UsuarioNoExiste,
    }
   
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone, PartialEq, Eq, Debug)]

    pub struct Usuario{
        nombre:String,
        apellido:String,
        email:String,
        id:AccountId,
        rol: Rol,
    }
    
    
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone, PartialEq, Eq, Debug)]

    pub enum Rol {
        Comprador,
        Vendedor,
        Ambos,
    }

    impl Sistema {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value, usuarios: Mapping::new() }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn registrar_usuario(&mut self, nombre:String, apellido:String, email:String, rol:Rol) -> Result<(), ErrorSistema> {
            let id = self.env().caller(); // Se obtiene el AccountId del usuario que llama a la función.

            self._registrar_usuario(nombre, apellido, email, rol, id)?;
            Ok(())
        }


        //Siempre lo marca como ya registrado (por más de que no lo esté) ????
        fn _registrar_usuario(&mut self, nombre:String, apellido:String, email:String, rol:Rol, id:AccountId) -> Result<(), ErrorSistema>{
            
            // Chequear que el usuario a registrar no exista en el sistema. (Solo registrar usuarios nuevos)
            if self.usuarios.get(&id).is_some() { //Busca match en el mapping.
                return Err(ErrorSistema::UsuarioYaRegistrado);
            }                
            
            self.usuarios.insert(id, &Usuario {nombre, apellido, email, id, rol});
            Ok(())
        }

        /*#[ink(message)]
        pub fn modificar_rol(&self, nuevo_rol:Rol)->Result<(), ErrorSistema>{
             self._modificar_rol(nuevo_rol)?
        }

        fn _modificar_rol(&mut self, nuevo_rol:Rol) {
            if let Some(user) = self.usuarios.get_mut(self.env().caller()){
                user.rol = nuevo_rol;
                return Ok(());
            }
            return Err(ErrorSistema::UsuarioNoExiste);
        }#[ink(message)]
        pub fn get_user(&self) -> Result<Usuario, ErrorSistema>{ // Result

            let _caller = self.env().caller(); // Se busca con el AccountId de la cuenta asociada.

            self._get_user(_caller) 
        }*/

        fn _get_user(&self, id:AccountId)-> Result<Usuario, ErrorSistema>{

            if let Some(user) = self.usuarios.get(&id) {
                Ok(user.clone())
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let sistema = Sistema::default();
            assert_eq!(sistema.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut sistema = Sistema::new(false);
            assert_eq!(sistema.get(), false);
            sistema.flip();
            assert_eq!(sistema.get(), true);
        }

        /// We test that we can register a user.
        /// In this test the user is added successfully.
        #[ink::test]
        fn registrar_usuario_okay() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new(true);

            assert!(sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador).is_ok());
        }

        /// We test that we cannot register a user that already exists.
        #[ink::test]
        fn registrar_usuario_not_okay() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new(true);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            assert!(sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador).is_err());
        }

        /// We test that the system is built correctly.
        /*#[ink::test]
        fn test_new() {
            let sistema = Sistema::new(true);
            assert_eq!(sistema.usuarios.len(), 0); //Why can't I use len() ???
        }*/
        
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = UsuariosRef::default();

            // When
            let contract = client
                .instantiate("usuarios", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Usuarios>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = UsuariosRef::new(false);
            let contract = client
                .instantiate("usuarios", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Usuarios>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}

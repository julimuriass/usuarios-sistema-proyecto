#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod usuarios_sistema {
    use ink::prelude::{format, string::String};
    use ink::storage::Mapping;    
    use ink::storage::StorageVec;

    #[ink(storage)]

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    pub struct Sistema {
        value: bool, //Tendríamos que dejar el value??
        usuarios: ink::storage::Mapping<AccountId, Usuario>,
        //historial_transacciones: ink::storage::StorageVec<transaccion>, //-> Hay que tener un struct para transaccion???
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Clone, PartialEq, Eq, Debug)]

    pub enum ErrorSistema {
        UsuarioYaRegistrado,
        UsuarioNoExiste,
        RolYaEnUso,
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
        //productos: Option<Producto>, //Si es vendedor tiene que tener una lista de sus productos.
        //orden_compra: Option<OrdenDeCompra>, //Si es comprador tiene que tener una orden de compra.
        //Duda: Tendría que tener un historial de sus propias transacciones?
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

        //Verificadores del sistema.
        fn _existe_usuario(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            if self.usuarios.get(&id).is_some() {
                Ok(true)
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }

        #[ink(message)]
        pub fn es_vendedor(&self) -> Result<bool, ErrorSistema> { //Duda: Está bien recibirlo como parámetro al id o lo tengo que obtener del caller?
            //Duda: Debería preguntar acá o en el privado si el usuario existe?

            let id = self.env().caller(); //Está bien esto? 
            self._es_vendedor(id)
        }

        fn _es_vendedor(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            //Si existe el usuario
                //lo encuentro
                //y verifico si es vendedor o ambos.
            //Si no existo -> ErrorSistema::UsuarioNoExiste

            if (self._existe_usuario(id)).is_err() {
                return Err(ErrorSistema::UsuarioNoExiste);
            } else {
                //Busco al usuario y verifico su rol.
                let user = self.usuarios.get(&id);
                match user.unwrap().rol {
                    Rol::Vendedor | Rol::Ambos => Ok(true),
                    _ => Ok(false),
                }   
            }
        }

        //Mismas dudas que en es_vendedor.
        #[ink(message)]
        pub fn es_comprador(&self) -> Result<bool, ErrorSistema> { 
            let id = self.env().caller(); 
            self._es_comprador(id)
        }

        fn _es_comprador(&self, id: AccountId) -> Result<bool, ErrorSistema> {
            //Si existe el usuario
                //lo encuentro
                //y verifico si es comprador o ambos.
            //Si no existo -> ErrorSistema::UsuarioNoExiste

            if (self._existe_usuario(id)).is_err() {
                return Err(ErrorSistema::UsuarioNoExiste);
            } else {
                //Busco al usuario y verifico su rol.
                let user = self.usuarios.get(&id);
                match user.unwrap().rol {
                    Rol::Comprador | Rol::Ambos => Ok(true),
                    _ => Ok(false),
                }   
            }
        }

        //Funciones asociadas a usuarios. 

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

        #[ink(message)]
        pub fn agregar_rol(&mut self, rol: Rol) -> Result<(), ErrorSistema> {
            let id = self.env().caller(); // Se obtiene el AccountId del usuario que llama a la función.

            self._agregar_rol(rol, id)
        }

        fn _agregar_rol(&mut self, rol: Rol, id: AccountId) -> Result<(), ErrorSistema> { //Hacer un agregar para cada rol distinto.
            // Verifica si el usuario existe.
            if let Some(mut user) = self.usuarios.get(&id) {  
                user.agregar_rol(rol.clone())?; //Llama a la función del usuario que modifica su rol. (Lo delega)
                self.usuarios.insert(&id, &user); //Lo guardo modificado en le mapping.
                Ok(())
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }


        fn _get_user(&self, id:AccountId)-> Result<Usuario, ErrorSistema>{

            if let Some(user) = self.usuarios.get(&id) {
                Ok(user.clone())
            } else {
                Err(ErrorSistema::UsuarioNoExiste)
            }
        }
    }

    impl Usuario {
        //registrarse. ?? Acá sí que no me quedó clara la parte de delegar.
        //pub fn crear_publicación
        //pub fn agregar_a_orden_compra
        
        pub fn agregar_rol(&mut self, rol: Rol) -> Result<(), ErrorSistema> { //Hacer un agregar para cada rol distinto..
            if self.rol == rol {
                return Err(ErrorSistema::RolYaEnUso);
            }
            // Agrega el nuevo rol al usuario.
            self.rol = match (self.rol.clone(), rol.clone()) {
                (Rol::Comprador, Rol::Vendedor) | (Rol::Vendedor, Rol::Comprador) => Rol::Ambos,
                _ => rol, //está de más?
            };
            Ok(())
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

        #[ink::test]
        fn test_existe_usuario() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new(true);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            assert!(sistema._existe_usuario(alice).is_ok());

            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);
            assert!(sistema._existe_usuario(bob).is_err());
        }

        #[ink::test]
        fn test_es_vendedor() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new(true);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Vendedor);

            //Pruebo con un usuario (alice) que esté en el sistema y sea vendedor.
            assert!(matches!(sistema.es_vendedor(), Ok(true)));

            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Comprador);

            //Pruebo con un usuario (charlie) que esté en el sistema pero no sea vendedor.
            assert!(matches!(sistema.es_vendedor(), Ok(false)));


            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            //Pruebo con un usuario (bob) que no esté en el sistema.
            assert!(sistema.es_vendedor().is_err());
        }

        #[ink::test]
        fn test_es_comprador() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new(true);
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            //Pruebo con un usuario (alice) que esté en el sistema y sea comprador.
            assert!(matches!(sistema.es_comprador(), Ok(true)));

            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Pruebo con un usuario (charlie) que esté en el sistema pero no sea vendedor.
            assert!(matches!(sistema.es_comprador(), Ok(false)));


            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            //Pruebo con un usuario (bob) que no esté en el sistema.
            assert!(sistema.es_comprador().is_err());
        }

        #[ink::test]
        fn test_agregar_rol() {
            let alice = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);

            let mut sistema = Sistema::new(true);
            //Inicializa alice como comprador.
            sistema.registrar_usuario(String::from("Alice"), String::from("Surname"), String::from("alice.email"), Rol::Comprador);

            //Se agrega el rol de vendedor (pasa a tener ambos).
            assert!(sistema.agregar_rol(Rol::Vendedor).is_ok());
            if let Some(user) = sistema.usuarios.get(&alice) {
                assert!(user.rol == Rol::Ambos);
            }
            //-----------------------------------------------------

            //Inicializa bob como vendedor.
            let bob = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);

            let mut sistema = Sistema::new(true);
            sistema.registrar_usuario(String::from("Bob"), String::from("Surname"), String::from("bob.email"), Rol::Vendedor);

            //Se agrega el rol de vendedor (pasa a tener ambos).
            assert!(sistema.agregar_rol(Rol::Comprador).is_ok());
            if let Some(user) = sistema.usuarios.get(&bob) {
                assert!(user.rol == Rol::Ambos);
            }

            //-----------------------------------------------------

            //Inicializa charlie como vendedor.
            let charlie = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(charlie);

            let mut sistema = Sistema::new(true);
            sistema.registrar_usuario(String::from("Charlie"), String::from("Surname"), String::from("charlie.email"), Rol::Vendedor);

            //Ya tiene el rol de vendedor. Por lo qe no se puede agregar el rol de vendedor otra vez..
            let error = sistema.agregar_rol(Rol::Vendedor).unwrap_err();
            assert_eq!(error, ErrorSistema::RolYaEnUso);

            //-----------------------------------------------------
            let eve = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().eve;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(eve);

            //Pruebo con un usuario (dave) que no esté en el sistema.
            let error = sistema.agregar_rol(Rol::Vendedor).unwrap_err();
            assert_eq!(error, ErrorSistema::UsuarioNoExiste);
        }
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

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod event_manager_contract {
    use ink::prelude::{string::String, vec::Vec};
    use parity_scale_codec::{Decode, Encode};

    #[derive(Debug, Decode, Encode, Clone, PartialEq)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo))]
    pub enum Status {
      Agendado,
      EmAndamento,
      Concluido
    }

    #[derive(Debug, Decode, Encode, Clone, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Evento {
        pub id: String,              
        pub nome: String,            
        pub participantes: u32,      
        pub data: String,            
        pub status: Status,          
    }

    #[ink(storage)]
    pub struct EventManagerContract{
      eventos: Vec<Evento>,
    }

    impl Default for EventManagerContract {
      fn default() -> Self {
          Self::default()
      }
  }

    impl EventManagerContract {
        #[ink(constructor)]
        pub fn default() -> Self{
          Self{
            eventos: Vec::new()
          }
        }

        #[ink(message)]
        pub fn adicionar_evento(
            &mut self,
            id: String,
            nome: String,
            participantes: u32,
            data: String,
            status: Status,
        ) {
            let evento = Evento {
                id,
                nome,
                participantes,
                data,
                status,
            };
            self.eventos.push(evento);
        }

        #[ink(message)]
        pub fn listar_eventos(&self) -> Vec<Evento> {
            self.eventos.clone()
        }

        #[ink(message)]
        pub fn atualizar_status(&mut self, id: String, novo_status: Status) -> bool {
            if let Some(evento) = self.eventos.iter_mut().find(|e| e.id == id) {
                evento.status = novo_status;
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn remover_evento(&mut self, id: String) -> bool {
            if let Some(pos) = self.eventos.iter().position(|e| e.id == id) {
                self.eventos.remove(pos);
                true
            } else {
                false
            }
        }
    }
    

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_adicionar_evento() {
            let mut contrato = EventManagerContract::new();

            contrato.adicionar_evento(
                "1".to_string(),
                "Evento de Teste".to_string(),
                100,
                "2025-01-01".to_string(),
                Status::Agendado,
            );

            let eventos = contrato.listar_eventos();
            assert_eq!(eventos.len(), 1);
            assert_eq!(eventos[0].id, "1");
            assert_eq!(eventos[0].nome, "Evento de Teste");
            assert_eq!(eventos[0].status, Status::Agendado);
        }

        #[test]
        fn test_listar_eventos_vazio() {
            let contrato = EventManagerContract::new();
            let eventos = contrato.listar_eventos();
            assert_eq!(eventos.len(), 0);
        }

        #[test]
        fn test_atualizar_status() {
            let mut contrato = EventManagerContract::new();

            contrato.adicionar_evento(
                "1".to_string(),
                "Evento de Teste".to_string(),
                100,
                "2025-01-01".to_string(),
                Status::Agendado,
            );

            let atualizado = contrato.atualizar_status("1".to_string(), Status::Concluido);
            assert!(atualizado);

            let eventos = contrato.listar_eventos();
            assert_eq!(eventos[0].status, Status::Concluido);
        }

        #[test]
        fn test_atualizar_status_id_inexistente() {
            let mut contrato = EventManagerContract::new();

            contrato.adicionar_evento(
                "1".to_string(),
                "Evento de Teste".to_string(),
                100,
                "2025-01-01".to_string(),
                Status::Agendado,
            );

            let atualizado = contrato.atualizar_status("2".to_string(), Status::Concluido);
            assert!(!atualizado);
        }

        #[test]
        fn test_remover_evento() {
            let mut contrato = EventManagerContract::new();

            contrato.adicionar_evento(
                "1".to_string(),
                "Evento de Teste".to_string(),
                100,
                "2025-01-01".to_string(),
                Status::Agendado,
            );

            let removido = contrato.remover_evento("1".to_string());
            assert!(removido);

            let eventos = contrato.listar_eventos();
            assert_eq!(eventos.len(), 0);
        }

        #[test]
        fn test_remover_evento_id_inexistente() {
            let mut contrato = EventManagerContract::new();

            contrato.adicionar_evento(
                "1".to_string(),
                "Evento de Teste".to_string(),
                100,
                "2025-01-01".to_string(),
                Status::Agendado,
            );

            let removido = contrato.remover_evento("2".to_string());
            assert!(!removido);

            let eventos = contrato.listar_eventos();
            assert_eq!(eventos.len(), 1);
        }
    }

}

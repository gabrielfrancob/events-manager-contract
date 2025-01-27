# Instruções para testar


## Listar os eventos
`cargo contract call --contract 5DiGy1BowH4kPwcd5KBe9tyg7sz79eT1v5gtUfcSvKNXFroy --message listar_eventos --suri //Bob --skip-dry-run --output-json`


## Criar evento
`cargo contract call --contract 5DiGy1BowH4kPwcd5KBe9tyg7sz79eT1v5gtUfcSvKNXFroy --message adicionar_evento --args "\"1\"" "\"Show do Stevan\"" 2 "\"11/02/2025\"" Agendado --suri //Bob --skip-confirm --execute`

## Atualizar status
`cargo contract call --contract 5DiGy1BowH4kPwcd5KBe9tyg7sz79eT1v5gtUfcSvKNXFroy --message atualizar_status --args "\"1\"" EmAndamento --suri //Bob --skip-confirm --execute`

## Remover evento
`cargo contract call --contract 5DiGy1BowH4kPwcd5KBe9tyg7sz79eT1v5gtUfcSvKNXFroy --message remover_evento --args 1 --suri //Bob --skip-confirm --execute`

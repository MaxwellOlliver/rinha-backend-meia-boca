## Rinha de backend meia bomba

### O que é a Rinha de backend meia bomba?

A ideia principal é que seja uma brincadeira entre amigos, se aventurando em uma linguagem completamente desconhecida. Criamos uma lista de linguagens que não sabemos NADA, e sorteamos. O importante é o conhecimento que ganhamos no final!

### Prazo (Edição 1)

Prazo 17/05/2024 - 24/05/2024

**Linguagem:**

-   [@odudas](https://github.com/odudas) - Python
-   [@MaxwellOlliver](https://github.com/MaxwellOlliver) - Rust

**Enunciado:**

O sistema deve permitir usuários de criarem uma conta, autenticarem no sistema, enviar transferências, consultar saldo e consultar extrato.

**Requisitos:**

-   Criar conta

-   Autenticação (JWT)

-   Enviar uma transferência

-   Consultar saldo

-   Consultar extrato

-   Depositar dinheiro

-   Sacar dinheiro

**Rotas**

Por padrão, todas as rotas que forem bem sucedidas devem retornar código 200. As não sucedidas, 422.

`POST /users`

Body

```json
{
	"name": "string",
	"password": "string",
	"email": "unique(string)"
}
```

Response (200)

```json
{
	"name": "string",
	"email": "string"
}
```

`POST /auth`

Body

```json
{
	"password": "string(8 - 32)",
	"email": "unique(string)"
}
```

Response (200)

```json
{
	"token": "string"
}
```

`POST /wallet/transfer/:userId`

Body

```json
{
	"amount": "number"
}
```

Response (200)

```json
{
	"balance": "number"
}
```

`POST /wallet/deposit`

Body

```json
{
	"value": "number"
}
```

Response (200)

```json
{
	"balance": "number"
}
```

`GET /wallet/balance`

Body

```json
{}
```

Response (200)

```json
{
	"balance": "number"
}
```

`GET /wallet/transactions`

Body

```json
{}
```

Response (200)

```json
{
	"balance": "number",
	"transactions": [
		{
			"id": "string",
			"userId": "string",
			"relatedUserId": "string",
			"type": "deposit | withdraw | transfer",
			"amount": "amount",
			"createdAt": "string"
		}
	]
}
```

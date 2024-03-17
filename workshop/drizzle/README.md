# Drizzle 

Drizzle ORM è un tool per la gestione del database.
In particolare, all'interno della cartella ``db`` si trova l'utilità per
l'implementazione delle interazioni col database.
Notiamo che per interagire con un database è necessario:

1. creare lo schema del database
2. effettuare la migrazione del database
3. creare le classi per interagire con il database
4. creare le query per interagire con il database

## Creare lo schema del database

All'interno della cartella ``db`` si trova il file ``schema.ts``. Questo file
contiene la definizione dello schema del database. In particolare, si 
definiscono le tabelle del database e le relazioni tra le tabelle. Per esempio:

```ts
export const utente = pgTable("utente", {
	id: serial("id").primaryKey(),
	email: varchar("email", { length: 255 }).unique().notNull(),
	username: varchar("username", { length: 100 }).notNull(),
	password: varchar("password", { length: 255 }).notNull(),
})
```

[elenco dei tipi di dato](https://orm.drizzle.team/docs/column-types/pg)

## Effettuare la migrazione del database

Per effettuare la migrazione del database si definisce un file ``migrate.ts``:

```ts
import { Pool } from 'pg';
import { drizzle } from 'drizzle-orm/node-postgres';
import { migrate } from 'drizzle-orm/node-postgres/migrator';

const pool = new Pool({
	connectionString: process.env.DATABASE_URL,
})

const db = drizzle(pool)

async function main() {
	console.log(process.env.DATABASE_URL)
	console.log("Migrating database...")
	await migrate(db, { migrationsFolder: "migrations" })
	console.log("Done!")
	process.exit(0)
}

main().catch(err => {
	console.error(err)
	process.exit(1)
})
```

Dopodiché si eseguono i comandi:

```bash
drizzle-kit generate:pg --schema=./src/db/schema.ts --out=migrations
node -r esbuild-register src/db/migrate.ts
```

In particolare, il primo comando genera le migrazioni, ovvero, crea dei file
.sql che contengono le query per creare le tabelle e le relazioni all'interno
della cartella ``migrations``. Il secondo comando esegue le migrazioni, ovvero,
esegue le query contenute nei file .sql.

## Creare le classi per interagire con il database

Avendo definito lo schema del database attraverso il file ``schema.ts``, le
classi per interagire con il database sono già create.  
In particolare, devono essere importate mediante il comando:

```ts
import { utente, ristorante } from '../db/schema'; // per importare le classi delle tabelle esplicitate nello schema
import { db } from '../db'; // per importare il database
import { eq } from 'drizzle-orm'; // per importare i metodi per le query
```

## Creare le query per interagire con il database

```ts
	async update(id: number, updateUtentiDto: UpdateUtentiDto): Promise<UpdateUtentiResultDto> {
		return db.update(utente)
			.set({ ...updateUtentiDto })
			.where(eq(utente.id, id))
			.returning()
			.then((utente) => {
				return {
					result: true,
					message: "Utente aggiornato con successo",
					utente: utente[0]
				}
			}).catch((_) => {
				return {
					result: false,
					message: "Errore nell'aggiornamento dell'utente",
					utente: null
				}
			})
	}
```

```ts
	async create(createUtentiDto: CreateUtentiDto): Promise<CreateUtentiResultDto> {
		return db.insert(utente)
			.values(createUtentiDto)
			.returning()
			.then((utente) => {
				return {
					result: true,
					message: "Utente creato con successo",
					utente: utente[0]
				}
			}).catch((_) => {
				return {
					result: false,
					message: "Errore nella creazione dell'utente",
					utente: null
				}
			})
	}
```

In ogni caso, per studiare la scrittura delle query si fa riferimento alla
pagina di drizzle: [drizzle queries](https://orm.drizzle.team/docs/queries)

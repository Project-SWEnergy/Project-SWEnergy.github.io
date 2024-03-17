# Che cos'è Nest.js?

Nest.js è un framework per costruire il back-end, ovvero l'applicativo che viene
eseguito sul server, basato su Node.js. In particolare Nest.js può essere usato
sia scrivendo in JavaScript che in TypeScript.  

## Che cos'è TypeScript?

JavaScript è il linguaggio di programmazione che è stato sviluppato per essere
semplice e per essere esguito dai browser. I browser sono come una macchina
virtuale, per cui gestiscono molte risorse del computer e si occupano di gestire
le connessioni https con i server. Le connessioni https sono eseguite in
JavaScript. In realtà, molti progetti stanno cercando di implementare 
[Web Assembly](https://webassembly.org/) sui browser, ma per il momento
JavaScript è più veloce, perché è stato ottimizzato e usato in modo molto più
intensivo e per molto più tempo. JavaScript è così performante perché qualche
pazzo ha scritto V8, l'engine, l'interprete che lo esegue. V8 è stato costruito
da Google e dunque Google Chrome lo usa per runnare i programmi JavaScript.
Il back-end che viene sviluppato in TypeScript e poi compilato in JavaScript è
fatto runnare da Node.js invece. Anche Node.js utilizza l'engine di Google.
JavaScript (js) è un linguaggio non tipizzato e interpretato. Questo vuol dire 
che è
molto semplice, anche la sua sintassi è di facile apprendimento, tuttavia, la
carenza dei tipi porta il linguaggio a comportarsi in modo strano ogni tanto.
JavaScript tenta di eseguire il codice ad ogni costo, per questo motivo, anche
se il codice sovrebbe crashare, continua a runnare invece, nascondendo le
eccezioni.  
Poiché JavaScript è ampiamente utilizzato sul web ed è diventato insostituibile,
Miscrosoft ha pensato di costruire TypeScript (ts) per aggiungere dei controlli.
TypeScript è un linguaggio fortemente tipizzato, in cui il tipo delle variabili
può essere derivato dal compilatore. Una volta scritto del codice in TypeScript,
questo deve essere compilato. Compilare del source code TypeScript vuol dire
convertire il suddetto source code in JavaScript. Durante la conversione il
compilatore effettua delle analisi statiche, per individuare gli errori e
ottimizzare il codice. Nota Bene, in JavaScript il codice non viene mai
controllato staticamente, quindi un programmatore si può accorgere di eventuali
errori solo leggendo il codice o eseguendo il codice (come python, ma meno di
python). In python, infatti, prima di runnare il codice, l'interprete esegue
un'analisi statica del codice e poi lo esegue.  
Dal momento che le code base di Microsoft erano piene di errori difficili da
individuare, il colosso ha deciso di sviluppare TypeScript in modo che fosse
compatibile con JavaScript.

## Nest.js

Tornando a noi. Nest.js è un framework per creare degli endpoint, delle api, da
associare a delle funzioni. Fondamentalmente, si collega il back-end ad un
indirizzo ip. Infine, si collegano dei path all'indirizzo ip, in modo tale da
poter ricevere delle richieste attraverso la connesione ``https``. Nota bene, da
qualche anno a questa parte (tipo uno o due), non si usano più le connessioni
``http``, che sono state sostituite da connessioni ``https``. Le protocollo
``https`` è compatibile con il predecessore. Per cui non è stato riscritto
assolutamente nulla per effettuare il cambiamento. Per quello che ci interessa,
il protocollo ``https`` offre la criptazione delle comunicazione come _built-in_
del protocollo. Vuol dire che uno dei requisiti facoltativi che ci era richiesto 
è soddisfatto.  
Per gestire gli endpoint Nest.js si basa su _express_, un framework per gestire
gli endpoint, ma può essere configurato per usato _fastify_, per migliorare le
performance. Potremmo informarci in futuro se è il caso di cambiare, per il 
momento impariamo ad usare Nest.js.

## Installazione

Dal momento che Nest.js è un framework per sviluppare del codice in JavaScript
si installa usanto il package manager di JavaScript: ```npm```:

```bash
npm i -g @nestjs/cli
RUN npm install --save @nestjs/swagger swagger-ui-express
RUN npm install drizzle-orm pg
```

Questo comando installa Nest.js nel computer. Ovviamente npm deve essere
disponibile per poter installare nest.  
Un progetto per utilizzare Nest.js viene inizializzato con il comando:

```bash
nest new project-name
```

A questo punto viene creata una cartella chiamata ``project-name`` e al suo
interno sono aggiunti un bel po' di file e di cartelle. Di base nest.js sviluppa
il progetto in modo tale che il programmatore scriva del codice TypeScript. 
Non descriverò tutti i file generati, ma solo alcuni di essi. Potete usare
ChatGPT per approfondire questo argomento (e ogni altro).

- Il file ``package.json`` corrisponde alle impostazioni del progetto. In questo
  file sono defiti i metadati del progetto, le dipendenze da altre librerie e gli
  script. Che cosa sono gli script? Gli script sono dei comandi definiti come

  ```json
  {
    "scripts": {
      "nome-script": "comando_da_eseguire"
    }
  }
  ```

  Il comando da eseguire deve essere nel linguaggio ``bash``, ovverosia, il
  linguaggio del terminale. Per eseguire uno script si scrive sul terminale,
  all'interno di una directory del progetto:
  
  ```bash
  npm run nome-script
  ```

  Per esempio per eseguire il server in locale viene eseguito il comando:

  ```bash
  npm run start
  ```

  Per eseguire il server in locale, in watch mode si usa il comando:

  ```bash
  npm run start:dev
  ```
  In watch mode, vuol dire che il programma viene ricompilato e runnato ogni
  volta che un file è aggiornato.
  
  Riassumento gli script sono delle shortcut del terminale, più o meno come gli
  [alias](https://www.howtogeek.com/439736/how-to-create-aliases-and-shell-functions-on-linux/).

- La cartella ``src``: nella cartella src si scrive il codice sorgente del
  backend, che viene poi compilato in JavaScript seguendo le impostazioni
  definite nel file ``tsconfig.json``
  Nella cartella ``src`` sono inseriti i file:
  - ``app.controller.ts``: viene creata una singola API di esempio. I controller
    in Nest.js sono i moduli in cui vengono definiti gli endpoint.
  - ``app.controller.spec.ts``: dove inserire gli unit test per i controller.
  - ``app.module.ts``: un modulo di esempio. I moduli in Nest.js servono per
    isolare e incapsulare del codice, in modo da ridurne le dipendenze. Possiamo
    immaginare i moduli come se fossero namespace in cpp.
  - ``app.service.ts``: qui sono definite le funzioni che sono eseguite dai
    controller, quando ricevono qualche chiamata ``https``.
  - ``main.ts``: indovinate un po'.

## Sviluppare degli endpoint

Per sviluppare qualche endpoint dalla ``working directory`` del progetto si usa
il comando:

```bash
nest g resource
```

Questo comando richiederà:
- il nome dell'endpoint, del folder e dei file da creare (il nome lo chiede una
  volta e viene usato per nominare quanto elencato)
- il layer di trasporto da usare: noi solo REST API
- se vogliamo autogenerare i CRUD entry point (tendenzialmente sì)

Nota bene:  
**C**reate, ovvero post request  
**R**ead, ovver get request  
**U**pdate, ovvero patch request  
**D**elete, ovvero delete request

## Swagger

Swagger è un tool per la documentazione delle API. In Nest.js è possibile
generare la documentazione delle API in modo automatico. Per farlo bisogna
configurare il modulo ``SwaggerModule``. Ovvero aggiungendo al main.ts:

```ts
	const app = await NestFactory.create(AppModule);
	const config = new DocumentBuilder()
		.setTitle('Easy Meal')
		.setDescription('The Easy Meal API description')
		.setVersion('0.1')
		.build();

	const options: SwaggerDocumentOptions = {
		operationIdFactory: (_: string, methodKey: string) => methodKey,
	};

	const document = SwaggerModule.createDocument(app, config, options);
	SwaggerModule.setup('api', app, document); // indichiamo l'indirizzo per la documentazione
```

In realtà, la documentazione prodotta non è propriamente automatica, ovvero,
rimane necessario aggiungere dei decoratori ai controller e alle classi per
implementarne la descrizione.

### Controller

Dopo aver creato un controller, bisogna includere i decoratori opportuni nel
file:

```ts
import { ApiTags, ApiResponse, ApiOperation } from '@nestjs/swagger';

@ApiTags('nome-dell-endpoint')
@Controller('path/dell/endpoint')
export class UtentiController {
	constructor(private readonly utentiService: UtentiService) { }

	@Post()                                             // metodo http
	@ApiOperation({ summary: '', description: '' })     // descrizione del metodo
	@ApiResponse({ status: 201, type: ResultClass })    // tipo di risposta
	async create(@Body() arg: Class): Promise<ResultClass> {
		return await this.utentiService.create(arg);
	}
}
```

### Classe

Dopo aver creato una classe bisogna includere i decoratori opportuni nel file:

```ts
import { ApiProperty } from '@nestjs/swagger';

export class Class {
    @ApiProperty()
    id: number;

    @ApiProperty()
    nome: string;

    @ApiProperty()
    cognome: string;
}
```

In questo modo, swagger può generare la documentazione delle API in modo
automatico.

## Drizzle ORM

Drizzle ORM è un tool per la gestione del database.
In particolare, all'interno della cartella ``db`` si trova l'utilità per
l'implementazione delle interazioni col database.
Notiamo che per interagire con un database è necessario:

1. creare lo schema del database
2. effettuare la migrazione del database
3. creare le classi per interagire con il database
4. creare le query per interagire con il database

### Creare lo schema del database

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

### Effettuare la migrazione del database

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

### Creare le classi per interagire con il database

Avendo definito lo schema del database attraverso il file ``schema.ts``, le
classi per interagire con il database sono già create.  
In particolare, devono essere importate mediante il comando:

```ts
import { utente, ristorante } from '../db/schema'; // per importare le classi delle tabelle esplicitate nello schema
import { db } from '../db'; // per importare il database
import { eq } from 'drizzle-orm'; // per importare i metodi per le query
```

### Creare le query per interagire con il database

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

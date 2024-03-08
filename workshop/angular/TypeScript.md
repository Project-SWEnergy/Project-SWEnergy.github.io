[[#Tipi e variabili]]
	- [[#Dichiarazione]]
	- [[#Tipi]]

[[#Funzioni]]
	- [[#Parametri]]
	- [[#Arrow functions (lambda)]]
	
[[#Caratteristiche]]
	- [[#Spread parameter]]
	- [[#Template string]]
	- [[#Generics]]

[[#Classi, interfacce, decorators]]
	- [[#Classi]]
	- [[#Interfacce]]
	- [[#Ereditarietà]]
	- [[#Decorators]]

[[#Tipi avanzati]]
[[#Successivo]]

Superset di JavaScript, permette di:
- Utilizzare i tipi migliorando la comprensione del codice e la sua integrazione con moduli differenti.
- Analizzare i tipi permette di rilevare errori in fase di compilazione.
- Permette l'utilizzo di classi, campi privati, interfacce.

Un file TypeScript ha estensione .ts.

Disponibili altre informazioni sul sito [TypeScript](https://www.typescriptlang.org/).

# Tipi e variabili 
## Dichiarazione
La dichiarazione delle variabili segue la stessa base di JavaScript e viene estesa con i tipi, inseriti dopo i duepunti.
La dichiarazione di una variabile è definita dalle keyword:
- **let**: variabile limitata al blocco o all'espressione in cui è stata dichiarata. Questo è più sicuro in quanto aiuta ad evitare problemi legati a variabili globali indesiderate. Non possono essere utilizzate prima della loro dichiarazione, l'accesso a una variabile let prima della sua dichiarazione genererà un errore.
- **var**: funzionante nell'intera funzione in cui è stata dichiarata. Se è dichiarata al di fuori di una funzione, è una variabile globale. Le variabili dichiarate con **var** sono "sollevate" (hoisted) all'inizio della funzione o dello scope, il che significa che possono essere usate prima della loro effettiva dichiarazione nel codice. Tuttavia, il loro valore rimane *undefined* fino a quando non vengono effettivamente inizializzate.
- **const**: variabile il cui valore non cambia.
Esempio in JS:

	// Con var
	function exampleVar() {
	  console.log(x); // undefined
	  var x = 5;
	  console.log(x); // 5
	}
	
	// Con let
	function exampleLet() {
	  console.log(y); // Error: Cannot access 'y' before initialization
	  let y = 10;
	  console.log(y); // 10
	}
Si consiglia di utilizzare maggiormente **let**.

## Tipi 

### any
Tipo dinamico definito a runtime.

	let distance: any;
	distance = '1000km';
	distance = 1000;
	const distances: any[] = ['1000km', 1000];

### string
Stringhe, si possono inserire variabili stringa al loro interno con ''${ }''.

	var brand: string = 'Chevrolet';
	var message: string = 'I just bought a new ${brand} car';

### number
Floating-point number

	const age: number = 7;
	const height: number = 5.6;

### boolean
Assume valori *true*, *false*.

	const isZeroGreaterThanOne: boolean = false;

### array
Vincolato al tipo assegnatogli, può contenere valori di tipo differente solo se ha tipo *any*.

	const brands: string[] = ['Chevrolet', 'Ford', 'General Motors'];
	const ages: number[] = [8, 5, 12, 3, 1];


### type
Tipo definito da utente.

	type Animal = 'Cheetah' | 'Lion';
	const animal: Animal = 'Cheetah';
	const animal: Animal = 'Turtle'; // Errore


### enum
Valori numerici a cui viene assegnato un nome, iniziano da 0 a meno che non si assegni un valore specifico al nome.
Si può estrarre il nome effettuando l'accesso tramite indice.

	enum Brands { Chevrolet, Cadillac, Ford, Buick, Chrysler, Dodge };
	const myCar: Brands = Brands.Cadillac; // Valore 1

	enum Brands { Chevrolet, Cadillac, Ford, Buick, Chrysler, Dodge };
	const myCarBrandName: string = Brands[1] // Valore 'Cadillac'

	enum BrandsReduced { Tesla = 1, GMC, Jeep };
	const myTruck: BrandsReduced = BrandsReduced.GMC; // Valore 2

	enum StackingIndex {
	 None = 0,
	 Dropdown = 1000,
	 Overlay = 2000,
	 Modal = 3000
	};
	const myBoxStacking: StackingIndex = StackingIndex.Dropdown; // Vaore 1000

### void
Assenza di tipo spesso usata nelle funzioni.

	function test(): void {
		 const a = 0;
	}


# Funzioni
Come in JS, ma gli si può assegnare un tipo statico.

	function sayHello(name: string): string {
		 return 'Fuck you, ' + name;
	}

Le funzioni anonime si definiscono nel seguente modo:

	const sayHello: (name: string) => string = function(name: string): string
	{
		 return 'Se le usi ti chiamo Guglielmo, ' + name;
	}


## Parametri
### Parametri opzionali
	function greetMe(name: string, greeting?: string): string {
		if (!greeting) {
			 greeting = 'Hello';
		 }
		 return greeting + ', ' + name;
	}
I parametri opzionali devono sempre essere inseriti come ultimi all'intenro della funzione.
Nel seguente caso, indipendentemente dalle intenzioni del programmatore, entrambi i parametri sono considerati obbligatori:

	function add(optional?: number, mandatory: string) {}

### Parametri di default
	function greetMe(name: string, greeting: string = 'Hello'): string {
		 return '${greeting}, ${name}';
	}
Anche in questo caso devono essere inseriti per ultimi.


### Rest
Vi è la possibilità di passare ad una funzione un numero illimitato di parametri sotto forma di array, seguendo la seguente sintassi:

	function greetPeople(greeting: string, ...names: string[]): string {
		return greeting + ', ' + names.join(' and ') + '!';
	}


## Arrow functions (lambda)
Solitamente sono funzioni con una singola istruzione:

	const double = x => x * 2;

Se richiedono più parametri:

	const add = (x, y) => x + y;

Se richiedono più istruzioni:

	const addAndDouble = (x, y) => {
		 const sum = x + y;
		 return sum * 2;
	}

Nelle lambda si ha il vantaggio di non perdere l'uso del *this*, nel seguente esempio si mostra come:

	function delayedGreeting1(name): void {
		 this.name = name;
		 this.greet = function(){
			 setTimeout(function() {
				 console.log('Hello ' + this.name);
			 }, 0);
		 }
	}
	function delayedGreeting2(name): void {
		 this.name = name;
		 this.greet = function() {
			 setTimeout(() => console.log('Hello ' + this.name), 0);
		 }
	}
	const greeting1 = new delayedGreeting1('John');
	const greeting2 = new delayedGreeting2('John');
	greeting1.greet(); // Nessuna stampa
	greeting1.greet(); // Stampa corretta


# Caratteristiche
## Spread parameter
Indicato con i tre puntini ( ... ) dentro il corpo di una funzione.
Usato per creare un nuovo oggetto a partire da uno esistente, mantenendo inalterato quello vecchio.

	const newItem = 3;
	const oldArray = [1, 2];
	const newArray = [...oldArray, newItem];

## Template string
Il seguente url risulta difficile da leggere:

	const url = 'http://path_to_domain' +
	 'path_to_resource' +
	 '?param=' + parameter +
	 '&param2=' + parameter2;
Si semplifica la gestione di una stringa complessa utilizzando i parametri direttamente al sui interno, tramite la sintassi ${ nomeParametro }:

	const url =
	'${baseUrl}/${path_to_resource}?param=${parameter}&param2={parameter2}';

## Generics
Il tipo non viene valutato fino all'utilizzo del metodo.

	function method<T>(arg: T): T {
		 return arg;
	}
	method<number>(1); // Ok
	method<string>(1); // Errore
	


# Classi, interfacce, decorators
## Classi
Tutto quello che è presente su Angular è una classe TypeScript.
Le classi contengono:
- **Membri**: dati pubblici o privati.
- **Costruttori**: possono aggiungere membri come nel caso di *isHybrid*, non è necessario inserire al suo interno l'associazione tra i campi del costruttore ed i membri della classe siccome avviene in automatico.
- **Metodi**: pubblici o privati.
- **Metodi statici**: non fanno riferimento all'oggetto ma alla classe, non possono accedere ai membri con il *this*.
-  **Property accessors**: getters e setters.

Esempio di classe:

	class Car {
		 private distanceRun: number = 0;
		 private color: string;
	 
		 constructor(private isHybrid: boolean, color: string = 'red') {
			 this.color = color;
		 }
		 constructor(public make: string, public model: string) {}
		 
		 getGasConsumption(): string {
			 return this.isHybrid ? 'Very low' : 'Too high!';
		 }
		 
		 drive(distance: number): void {
			 this.distanceRun += distance;
		 }
		 
		 static honk(): string {
			 return 'HOOONK!';
		 }
		 
		 get distance(): number {
			 return this.distanceRun;
		 }
	}

## Interfacce
Le usiamo per definire un set minimo di dati che devono essere presenti nelle diverse classi che la estendono.
Possono contenere parametri opzionali.
Possono includere firme di metodi che verranno implementati successivamente.

	interface Exception {
		 message: string;
		 id?: number;
	}
	interface ErrorHandler {
		 exceptions: Exception[];
		 logException(message: string, id?: number): void
	}
	interface ExceptionHandlerSettings {
		 logAllExceptions: boolean;
	}
	
	class CustomErrorHandler implements ErrorHandler {
		 exceptions: Exception[] = [];
		 logAllExceptions: boolean;
	 
		 constructor(settings: ExceptionHandlerSettings) {
			 this.logAllExceptions = settings.logAllExceptions;
		 }
	 
		 logException(message: string, id?: number): void {
			 this.exceptions.push({message, id });
		 }
	}

## Ereditarietà
Una classe estende una classe padre, il popolamento dei membri del padre avviene tramite metodo *super*.
Inserire nella classe figlio un metodo della classe padre lo sovrascrive, quello della classe padre può essere usato tramite metodo *super*.

	class Sedan extends Car {
		 model: string;
		 
		 constructor(make: string, model: string) {
			 super(make);
			 this.model = model;
		 }
	}

## Decorators
Inserimento di metadati nelle classi, implementano nuove funzionalità senza creare nuove classi o sottoclassi.
Vengono riconosciuti dal prefisso @ e sono di quattro tipi:
- Classe.
- Proprietà.
- Metodo.
- Parametro.

### Classe
La classe *FruitBasket* viene decorata con la classe *Banana*, in tal modo è possibile invocare il metodo *banana()*.

	function Banana(target: Function): void {
		 target.prototype.banana = function(): void {
		 console.log('We have bananas!');
		  }
	}
	@Banana
	class FruitBasket {}
	const basket : any = new FruitBasket();
	basket.banana();

Questo decorator accetta una funzione target (la funzione costruttrice della classe) e modifica il suo prototipo aggiungendo un metodo chiamato *banana()*.
La classe *FruitBasket* ha il decorator *Banana* applicato. 
Ciò significa che quando viene istanziato un'oggetto di questa classe, erediterà il metodo *banana()* definito dal decorator.

### Proprietà
Applicati ad una classe e definiti creando una funzione che necessita di due parametri: *target*, *key*.
Target serve ad identificare l'oggetto su cui verrà applicato il decoratore, key è il parametro che viene passato a tale oggetto.

	function Jedi(target: Object, key: string) {
		 let propertyValue: string = target[key];
		 if (delete target[key]) {
			 Object.defineProperty(target, key, {
				 get: function() {
					 return propertyValue;
				 },
				 set: function(newValue){
					 propertyValue = newValue;
					 console.log('${propertyValue} is a Jedi');
				 }
			 });
		 }
	}
	class Character {
		 @Jedi
		 name: string;
	}
	const character = new Character();
	character.name = 'Luke';

In questo esempio la funzione *Jedi* è un decorator utilizzato per decorare la proprietà *name* della classe *Character*.
Il comportamento del decorator viene eseguito quando si accede o si modifica tale proprietà.
Al suo interno si ha *Object.defineProperty* per definire la proprietà dell'oggetto *target*, il getter restituisce il valore della proprietà mentre il setter lo modifica e stampa il valore su console.
La riga *delete target[key]* serve a rimuovere la proprietà *name* dall'oggetto *Character*, se non venisse fatto si avrebbero conflitti con *defineProperty*.

### Metodo
Interviene nell'esecuzione dei metodi.

	function Log(){
		 return function(target, key: string, descriptor: PropertyDescriptor) {
			 const oldMethod = descriptor.value;
			 descriptor.value = function newFunc(...args:any[]){
				 let result = oldMethod.apply(this, args);
				 console.log('${key} is called with ${args.join(',')} and result ${result}');
				 return result;
			 }
		}
	}
	class Hero {
		@Log()
		attack(...args:[]) { return args.join(); }
	}
	const hero = new Hero();
	hero.attack();

Il decorator è attaccato al metodo *attack()*.
All'interno del decorator, viene memorizzato il vecchio metodo originale e viene sovrascritto con una nuova funzione. 
Questa nuova funzione sostituisce il metodo originale, registrando i parametri di input e il risultato della chiamata nella console.
Ogni volta che viene chiamato il metodo attack, il decorator registrerà i dettagli della chiamata.

### Parametro
Ha effetto sui parametri presenti nelle firme delle funzioni, solitamente usato per logging o replicazione delle informazioni.

	function Log(target: Function, key: string, index: number) {
		 const functionLogged = key || target.prototype.constructor.name;
		 console.log('The parameter in position ${index} at ${functionLogged} has been decorated');
		}
	class Greeter {
		 greeting: string;
		 constructor (@Log phrase: string) {
			 this.greeting = phrase;
		 }
	}

Il decorator *Log* è applicato al parametro *phrase* del costruttore della classe *Greeter*. 
Quando viene istanziata un'oggetto della classe *Greeter*, il decorator viene eseguito e stampa un messaggio nella console indicando che il parametro alla posizione *index* è stato decorato.


# Tipi avanzati

### Partial
Usato per creare oggetti da interfacce, escludendone alcune proprietà.
Il seguente esempio include solo la proprietà *name*.
	
	interface Hero {
		 name: string;
		 power: number;
	}
	const hero: Partial<Hero> = {
		 name: 'Boothstomper'
	}

### Record
Equivalente ai dizionari, coppie chiave valore. 
Nell'esempio si registra la chiave come stringa ed il valore come numerico.

	interface Hero {
		 powers: Record<string, number>
	}

### Union
Permettono di definire diverse possibilità per i tipi, come *any* ma più limitato.
Nel seguente esempio la variabile *powers* accetta sia un array di numeri che un Record.

	interface Hero {
		 name: string;
		 powers: number[] | Record<string, number>;
	}


# Successivo
Continua con Angular su [[Componenti]]
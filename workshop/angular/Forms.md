# Forms
## Creazione
Selezioniamo il componente su cui verrà visualizzato il form.
Importiamo i moduli necessari all'utilizzo dei form e creiamo una variabile di tipo *FormGroup*.

	import { Component, inject } from '@angular/core';
	import { ActivatedRoute } from '@angular/router';
	import { HousingLocation } from '../../interfacce/housing-location';
	import { HousingService } from '../../servizi/housing.service';
	import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';
	
	@Component({
	  selector: 'app-details',
	  standalone: true,
	  imports: [ReactiveFormsModule],
	  templateUrl: './details.component.html',
	  styleUrl: './details.component.css'
	})
	export class DetailsComponent {
	  route: ActivatedRoute = inject(ActivatedRoute);
	  housingService = inject(HousingService);
	  housingLocation: HousingLocation | undefined;
	  applyForm = new FormGroup({
	    firstName: new FormControl(''),
	    lastName: new FormControl(''),
	    email: new FormControl('')
	  }
	  )
	  constructor() {
	    const housingLocationId = Number(this.route.snapshot.params["id"])
	    this.housingLocation = this.housingService.getHousingLocationById(housingLocationId)
	  }
	}

- **FormGroup**: classe in Angular che rappresenta un gruppo di controlli del modulo (form). 
- **FormControl**: classe che rappresenta un singolo controllo del modulo (form), come ad esempio un campo di input. 

A questo punto modifichiamo la componente HTML inserendo il form:

	<article>
	    <section>
	        <h2>Apply</h2>
	        <form [formGroup]="applyForm" (submit)="submitApplication">
	            <label for="first-name">First name</label>
	            <input id="first-name" 
	                type="text"  
	                formControlName="firstName">
	            <label for="last-name">Last name</label>
	            <input id="last-name" 
	                type="text"  
	                formControlName="lastName">
	            <label for="email">Email</label>
	            <input id="email" 
	                type="text"  
	                formControlName="email">
	            <button type="submit">Apply</button>
	        </form>
	    </section>
	</article>

La porzione di codice 

	(submit)="submitApplication"

indica che la funzione *submitApplication* dovrà essere richiamata al verificarsi dell'evento *submit*, descritto in [[#Invio di dati]].


## FormControl
### Valore iniziale
Il valore iniziale del controllo, può essere una stringa, un numero, un booleano, ecc.
Questo significa che quando il modulo viene renderizzato, il campo di input associato a questo controllo avrà il testo preimpostato "valore iniziale". 
L'utente può poi modificare questo valore attraverso l'interazione con il campo di input.

	new FormControl('valore iniziale')

### Validator
I validatori sincroni sono funzioni che prendono il controllo come argomento e restituiscono un oggetto di errore se la validazione non ha successo o null se la validazione ha successo.
Nel seguente esempio il valore di default è vuoto, inoltre:
- il controllo sul nome lo rende un campo obbligatorio
- il controllo sull'email verifica la sintassi corretta relativa alle email e la rende obbligatoria

Si possono inoltre mostrare nella console i relativi messaggi di errore.

	import { FormControl, Validators } from '@angular/forms';
	const controlloNome = new FormControl('', Validators.required);
	const controlloEmail = new FormControl('', [Validators.required, Validators.email]);
	console.log(controlloEmail.valid); 
	console.log(controlloEmail.errors);

Di seguito è riportato un esempio di validatori predefiniti:

	const controllo = new FormControl('', Validators.required);
	const controllo = new FormControl('', Validators.minLength(3));
	const controllo = new FormControl('', Validators.maxLength(10));
	const controllo = new FormControl('', Validators.pattern(/[a-zA-Z]/));
	const controlloEmail = new FormControl('', Validators.email);
	const controllo = new FormControl('', Validators.min(5));
	const controllo = new FormControl('', Validators.max(100));

### asyncValidator
A differenza dei validatori sincroni, i validatori asincroni eseguono operazioni asincrone come chiamate HTTP per verificare la validità del campo di input.
#todo 

### updateOn
Questo parametro consente di controllare quando il valore del controllo dovrebbe essere aggiornato e quando dovrebbe essere eseguita la validazione.

	import { FormControl } from '@angular/forms';
	const controlloNome = new FormControl('', { updateOn: 'blur' });
	controlloNome.valueChanges.subscribe((nuovoValore) => {
	  console.log('Nuovo valore durante blur:', nuovoValore);
	});

Questo codice indica che il controllo dovrebbe essere aggiornato quando l'utente esce dal campo di input, ovvero quando il campo perde il focus (evento "blur").
I parametri accettati sono:
- **change**: aggiorna il controllo ogni volta che il valore del campo di input cambia.
- **blur**: aggiorna il controllo quando l'utente esce dal campo di input (perde il focus).
- **submit**: aggiorna il controllo quando il modulo viene accettato (ad esempio, quando viene premuto un pulsante di invio).
L'utilizzo di *updateOn* è utile per ottimizzare le prestazioni e migliorare l'esperienza utente, poiché consente di evitare aggiornamenti e validazioni continui durante ogni singolo input, focalizzandoli solo sugli eventi desiderati.

### disabled
Utilizzato per specificare se il controllo del modulo deve essere inizializzato come disabilitato o meno. 
Se il parametro è impostato su true allora il controllo sarà disabilitato impedendo all'utente di cambiarne il valore predefinito, se è impostato su false (o non è specificato), il controllo sarà abilitato.
Si può abilitare o disabilitare dinamicamente il controllo.

	const controlloNome = new FormControl({ value: 'Valore iniziale', disabled: true });
	controlloNome.enable();
	controlloNome.disable();


## Invio di dati
Nel corpo della classe della componente che ospita il form dobbiamo definire la funzione *submitApplication*, descritta in [[#Creazione]].

	submitApplication() {
	    this.housingService.submitApplication(
	      this.applyForm.value.firstName ?? '',
	      this.applyForm.value.lastName ?? '',
	      this.applyForm.value.email ?? ''
	    )
	  }

Questa funzione richiama il servizio definito in [Services](workshop/angular/Services.html) che dovrà essere esteso per comprendere la funzione *submitApplication*.

## Successivo
[HTTP](workshop/angular/HTTP.html)
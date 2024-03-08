# Interfacce
Definiscono la struttura dei dati che verranno inviati e visualizzati

	ng generate interface interfacce/HousingLocation

Nella cartella *interfacce* viene generato un file TS, vi inseriamo il nome dei dati ed il loro tipo come segue:

	export interface HousingLocation {
		  id: number;
		  name: string;
		  city: string;
		  state: string;
		  photo: string;
		  availableUnits: number;
		  wifi: boolean;
		  laundry: boolean;
	}

A questo punto andiamo nel file TS dove si richiede l'utilizzo dell'interfaccia e modifichiamo l'import con il seguente codice:

	import { Component } from '@angular/core';
	import { HousingLocation } from '../../interfacce/housing-location';

Ora modifichiamo il corpo della classe *HousingLocationComponent*  come segue:

	export class HousingLocationComponent {
	  `@Input() housingLocation!: HousingLocation;`
	}

Il punto esclamativo indica che il parametro non sarà NULL o Undefined.
Ora è possibile modificare il file HTML della componente che utilizzerà i dati, si può trattare la variabile *housingLocation* come un JSON e accedere ai suoi campi:

	<section class="listing">
	    <img class="listing-photo" 
	        [src]="housingLocation.photo"
	        alt=" Foto di {{housingLocation.name}}">
	    <h2 class="listing-heads"> {{ housingLocation.name}} </h2>
	    <p class="listing-location"> {{housingLocation.state}} </p>
	</section>  

Ora dobbiamo includere dati, dunque nella classe della componente che conterrà i vari set di JSON avremo:

	export class HomeComponent {
		  housingLocationList: HousingLocation[] = []
	}

Dentro le parentesi quadre indichiamo il JSON contenente i dati effettivi da inserire nella pagina.

Ora bisogna importare tali dati nella pagina web.
Abbiamo creato una componente che descrive la visualizzazione di una delle "cards" contenente i dati, ci spostiamo nel documento che include il tag HTML di tale componente e lo modifichiamo come segue:

	<app-housing-location 
        *ngFor="let housingLocation of housingLocationList"
        [housingLocation] = "housingLocation" >
    </app-housing-location>

In questo codice si usa la direttiva strutturale di Angular chiamata *ngFor*, che viene utilizzata per iterare su un elenco di elementi. 
In questo caso stiamo iterando sull'array *housingLocationList* e assegnando ogni elemento a una variabile temporanea chiamata *housingLocation*.

	[housingLocation]="housingLocation" 
	
Questo è un binding di input. Stiamo passando il valore corrente dell'iterazione al componente app-housing-location come input con il nome housingLocation.
Questo codice crea dinamicamente un'istanza del componente *app-housing-location* per ogni elemento nell'array *housingLocationList*, passando l'oggetto corrispondente come input al componente. 

## Successivo
[Routing](workshop/angular/Routing.html)
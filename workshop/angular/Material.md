# Installazione

Utilizzare il comando per aggiungere le componenti richieste:

	ng add @angular/material

Sarà data la possibilità di selezionare un tema tra quelli predefiniti.

Il comando effettuerà le seguenti modifiche:
- Inserimento dependencies in *package.json*
- Inserimento font ed icone in *index.html*
- Inserimento stile CSS globale


# Utilizzo

Il modulo che si vuole utilizzare deve essere importato, fare riferimento a https://material.angular.io/components/categories per informazioni più dettagliate

File TS:

	import { MatSlideToggleModule } from '@angular/material/slide-toggle';
	
	@NgModule ({
	  imports: [ MatSlideToggleModule ]
	})
	class AppModule {}

File HTML:

	<mat-slide-toggle>Toggle me!</mat-slide-toggle>

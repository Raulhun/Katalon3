<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>select_Szakrendeles</name>
   <tag></tag>
   <elementGuidId>c4c10477-36c2-4556-a880-b0208f8a2019</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//select[@id='elojegyzes-felhasznalo']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>select</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>elojegyzes-felhasznalo</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>size</name>
      <type>Main</type>
      <value>6</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onchange</name>
      <type>Main</type>
      <value>
						console.log('- - - START TRIGGER - - -')
						var my_values = $(this).val()
						for(var i = 0; i &lt; my_values.length; i++) {
							if(window.__selected_surgeries.indexOf(my_values[i]) == -1) {
								window.__selected_surgeries.push(my_values[i])
							}
						}
						
						while(my_values.length !== window.__selected_surgeries.length) {
							for(var i = 0; i &lt; window.__selected_surgeries.length; i++) {
								if(my_values.indexOf(window.__selected_surgeries[i]) == -1) {
									window.__selected_surgeries.splice(i, 1);
								}
							}
						}

						var ids = window.__selected_surgeries.join(',')
						
$('input[name=vizsgalat-kereses]').val('')
setComponentInput('elojegyzes-heti', 'cols', 'FelhasznaloSzakrendelesID', ids)
setComponentGroupInput('elojegyzes-heti', 'reservations', 'FelhasznaloSzakrendelesID', ids)
setComponentValueInput('elojegyzes-heti', 'reservations', 'FelhasznaloSzakrendelesID', ids)
setComponentGroupInput('elojegyzes-heti', 'available', 'FelhasznaloSzakrendelesID', ids)
setComponentValueInput('elojegyzes-heti', 'available', 'FelhasznaloSzakrendelesID', ids)
$$('elojegyzes-szabad').addClass('loading')
$$('szabad-idopontok').removeClass('hidden')
$$('heti-nezet').addClass('hidden')
refreshComponent('elojegyzes-heti', function(){
$$('elojegyzes-heti').removeClass('hidden')
$$('elojegyzes-szabad').removeClass('loading').addClass('hidden')
szurkulet()
})
$('#uj-elojegyzes').addClass('disabled')
$('#szabad-idopontok').addClass('disabled')
setComponentGroupInput('vizsgalat', 'options', 'FelhasznaloSzakrendelesID', ids)
refreshComponent('vizsgalat')</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	TTTTTTTTTTTTTTT - 3
	tetetete - 3
	Teszt Felhasználó - 3
	Teszt Elek 2 - 3
	sfafdsafdf - 3
	prrrr - 3
	Próbaaaa - 3
	Miszter URL - 3
	Még újabb felhasználó - 3
	Legújabb Doktorúr - 3
	Krisztian Vasas - 3
	gyulus - 3
	Dr. Teszt Elek - 3
	B.C. - 3
	a felhasznalo aladar - 3
	a - 3
	Bármely orvos - 3
	sfafdsafdf - Avar utca
	Dr. Psyklon - Avar utca
	Bármely orvos - Avar utca
	Tireal - Blck
	Dr. Josef Mengele - Kardiológia
	Új felhasználó - Nőgyógyászat
	Teszt Elek - Nőgyógyászat
	Ragnaros - Nőgyógyászat
	Dr. Psyklon - Nőgyógyászat
	Dr. Josef Mengele - Nőgyógyászat
	Dr. Bekéne Antal - Nőgyógyászat
	Bármely orvos - Nőgyógyászat
	Teszt Elek 2 - Rendelés
	Ragnaros - Rendelés
	Dr. Visi László - Rendelés
	Dr. Psyklon - Rendelés
	Bármely orvos - Rendelés
	Dr. Psyklon - Sebészet
	Dr. Brokvik Ulrich Hanburber Béla - Teljestest átalakítás
	Új felhasználó - Új szakrendelés
	Tireal - Új szakrendelés
	Teszt Elek - Új szakrendelés
	Bármely orvos - Új szakrendelés
	Dr. Psyklon - Újnál is újabb rendelés
	Dr. Brokvik Ulrich Hanburber Béla - Újnál is újabb rendelés
	Bármely orvos - Újnál is újabb rendelés
	Dr. Psyklon - Urológia
	Dr. Brokvik Ulrich Hanburber Béla - Urológia
	Bármely orvos - Urológia
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;elojegyzes-felhasznalo&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//select[@id='elojegyzes-felhasznalo']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='elojegyzes-tab']/div/div[3]/select</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Szakrendelések:'])[1]/following::select[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Módosítások mentése'])[1]/following::select[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Vizsgálatok:'])[1]/preceding::select[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//select</value>
   </webElementXpaths>
</WebElementEntity>

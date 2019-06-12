<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>modal_email_sablon</name>
   <tag></tag>
   <elementGuidId>91206d56-7e6e-4fb1-91cd-ca7b417c77b4</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='email-sablonok']/div</value>
      </entry>
      <entry>
         <key>BASIC</key>
         <value>id(&quot;email-sablonok&quot;)/div[@class=&quot;inner&quot;]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>inner</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


E-mail sablon

	&lt;Új sablon>
	100 Folk
	asdasdasdasdadasdasw
	Citológiai email csatolmány P1
	Citológiai email csatolmány P2
	Citológiai kiértesítés
	Csatolmányos
	Csövi vizsgálat kiértesítés
	ed
	Előjegyzés online
	Email sablon vizsgálathoz
	kruzg
	Némileg új általános sablon
	nogyogy_negativ
	Nyulak
	Nyulak 2
	oujlk
	ÚJ TESZT 2


components['email-sablon-select']={&quot;label&quot;:&quot;E-mail sablon&quot;,&quot;class&quot;:&quot;small-label&quot;,&quot;defaults&quot;:{&quot;-1&quot;:&quot;&lt;\u00daj sablon>&quot;},&quot;options&quot;:[&quot;$client&quot;,&quot;listEmailSablon&quot;,&quot;EmailSablonok&quot;,{&quot;dbName&quot;:&quot;B9019003&quot;},&quot;EmailSablonID&quot;],&quot;format&quot;:true,&quot;name&quot;:&quot;{Nev}&quot;,&quot;metadata&quot;:{&quot;data-name&quot;:&quot;$Nev&quot;,&quot;data-type&quot;:&quot;$Tipus&quot;,&quot;data-subject&quot;:&quot;$Targy&quot;,&quot;data-text&quot;:&quot;$Szoveg&quot;,&quot;data-beav&quot;:&quot;$BeavatkozasID&quot;},&quot;onchange&quot;:&quot;changeEmailSablon(this)&quot;,&quot;_component&quot;:&quot;select&quot;,&quot;_hashname&quot;:&quot;_0e5dedd730b9c216da8e7c1f93ebea58&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-select&quot;};componentsLoaded['email-sablon-select']={};Object.assign(componentsLoaded['email-sablon-select'], components['email-sablon-select']);

	Név
	
	

components['email-sablon-nev']={&quot;name&quot;:&quot;email-sablon-nev&quot;,&quot;input&quot;:{&quot;label&quot;:&quot;N\u00e9v&quot;,&quot;labelSize&quot;:&quot;small&quot;,&quot;required&quot;:true,&quot;onchange&quot;:&quot;checkEmailSablonFields()&quot;},&quot;_component&quot;:&quot;inputs.text&quot;,&quot;_hashname&quot;:&quot;_b082dba326042adbb66f8d079ad70ded&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-nev&quot;};componentsLoaded['email-sablon-nev']={};Object.assign(componentsLoaded['email-sablon-nev'], components['email-sablon-nev']);

	Tárgy
	
	

components['email-sablon-targy']={&quot;name&quot;:&quot;email-sablon-targy&quot;,&quot;input&quot;:{&quot;label&quot;:&quot;T\u00e1rgy&quot;,&quot;labelSize&quot;:&quot;small&quot;,&quot;required&quot;:true,&quot;onchange&quot;:&quot;checkEmailSablonFields()&quot;},&quot;_component&quot;:&quot;inputs.text&quot;,&quot;_hashname&quot;:&quot;_e1a3d1ddd40b9341f33443029131dbe9&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-targy&quot;};componentsLoaded['email-sablon-targy']={};Object.assign(componentsLoaded['email-sablon-targy'], components['email-sablon-targy']);

Változó beszúrása Bekezdés 12px  
components['email-sablon-szoveg']={&quot;name&quot;:&quot;email-sablon-szoveg&quot;,&quot;input&quot;:{&quot;id&quot;:&quot;editor&quot;,&quot;class&quot;:&quot;tinymce&quot;,&quot;nowrap&quot;:&quot;true&quot;},&quot;_component&quot;:&quot;inputs.textarea&quot;,&quot;_hashname&quot;:&quot;_a02cf9733718c2aed202a7fd0e2f7c8b&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-szoveg&quot;};componentsLoaded['email-sablon-szoveg']={};Object.assign(componentsLoaded['email-sablon-szoveg'], components['email-sablon-szoveg']);


Sablon típusa

	Általános
	Visszaigazolás előjegyzésnél (globális)
	Visszaigazolás előjegyzésnél (vizsgálatonként)
	Citológiai értesítő
	Citológiai értesítő csatolmánya


components['email-sablon-tipus']={&quot;label&quot;:&quot;Sablon t\u00edpusa&quot;,&quot;class&quot;:&quot;small-label&quot;,&quot;onchange&quot;:&quot;checkSablonTipus()&quot;,&quot;options&quot;:{&quot;altalanos&quot;:&quot;\u00c1ltal\u00e1nos&quot;,&quot;globalis&quot;:&quot;Visszaigazol\u00e1s el\u0151jegyz\u00e9sn\u00e9l (glob\u00e1lis)&quot;,&quot;vizsgalatonkent&quot;:&quot;Visszaigazol\u00e1s el\u0151jegyz\u00e9sn\u00e9l (vizsg\u00e1latonk\u00e9nt)&quot;,&quot;cito_email&quot;:&quot;Citol\u00f3giai \u00e9rtes\u00edt\u0151&quot;,&quot;cito_csatolmany&quot;:&quot;Citol\u00f3giai \u00e9rtes\u00edt\u0151 csatolm\u00e1nya&quot;},&quot;_component&quot;:&quot;select&quot;,&quot;_hashname&quot;:&quot;_1f433fdc5eabbb616242ee8abde980a5&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-tipus&quot;};componentsLoaded['email-sablon-tipus']={};Object.assign(componentsLoaded['email-sablon-tipus'], components['email-sablon-tipus']);



	Vizsgálatok
	
		- - - - - - -
		Összes hozzáadása
		100 folk celsius koncert (Rendelés - Dr. Psyklon)
		100 minute (Rendelés - Dr. Psyklon)
		20 (Rendelés - Dr. Psyklon)
		20 (Rendelés - Dr. Psyklon)
		3892zr6ö (Rendelés - Dr. Psyklon)
		3rf34f (3 - a)
		5 perces (3 - TTTTTTTTTTTTTTT)
		55 perces (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		5min (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		5p (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		a (Rendelés - Dr. Psyklon)
		a régi (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		agyátültetés (3 - TTTTTTTTTTTTTTT)
		agyátültetés (Nőgyógyászat - Dr. Psyklon)
		agyátültetés (Rendelés - Dr. Psyklon)
		agyzsugorítás (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		alt (Sebészet - Dr. Psyklon)
		asd (3 - a)
		asd (Rendelés - Dr. Psyklon)
		asd (Rendelés - Dr. Psyklon)
		asd (Rendelés - Dr. Psyklon)
		asd (Teljestest átalakítás - Dr. Brokvik Ulrich Hanburber Béla)
		asdadsdsa (3 - a)
		asddasds (3 - a)
		Asdeff (3 - Dr. Teszt Elek)
		asdsadsa (3 - a)
		asdsdsa (3 - a)
		asfsa (Rendelés - Dr. Psyklon)
		b (Rendelés - Dr. Psyklon)
		By Fire Be Purged! (Rendelés - Ragnaros)
		c (Rendelés - Dr. Psyklon)
		csövi vizsgálat (Avar utca - Dr. Psyklon)
		csövi vizsgálat (Avar utca - Dr. Psyklon)
		csövi vizsgálat (Avar utca - Dr. Psyklon)
		csövi vizsgálat (Avar utca - Dr. Psyklon)
		csövi vizsgálat (Avar utca - Dr. Psyklon)
		csövi vizsgálat (Rendelés - Dr. Psyklon)
		d (Rendelés - Dr. Psyklon)
		dasdas (Rendelés - Dr. Psyklon)
		def (3 - a)
		delete (Sebészet - Dr. Psyklon)
		EBÉDSZÜNET (Sebészet - Dr. Psyklon)
		első vizit (Nőgyógyászat - Dr. Bekéne Antal)
		érvágás (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		fikatörlés (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		fülletépés (Teljestest átalakítás - Dr. Brokvik Ulrich Hanburber Béla)
		fülmosás (Rendelés - Dr. Psyklon)
		Gyógytorna (Rendelés - Dr. Psyklon)
		hereátültetés (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		karóba húzás (Teljestest átalakítás - Dr. Brokvik Ulrich Hanburber Béla)
		Kék (Rendelés - Dr. Psyklon)
		Kék vizsgálat 20 (Rendelés - Dr. Psyklon)
		kerékbe törés (Teljestest átalakítás - Dr. Brokvik Ulrich Hanburber Béla)
		kéz-láb levétel (Teljestest átalakítás - Dr. Brokvik Ulrich Hanburber Béla)
		kísérlet 1 (Nőgyógyászat - Dr. Josef Mengele)
		kontroll (Nőgyógyászat - Dr. Bekéne Antal)
		kontroll (Nőgyógyászat - Dr. Psyklon)
		kontroll (Sebészet - Dr. Psyklon)
		lefejezés (Teljestest átalakítás - Dr. Brokvik Ulrich Hanburber Béla)
		Másik próba (Rendelés - Dr. Psyklon)
		piláteszes karate (Rendelés - Dr. Psyklon)
		Piláteszes Teszt (Rendelés - Dr. Psyklon)
		pöcsmetszés (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		Próba (Nőgyógyászat - Teszt Elek)
		Próba (Rendelés - Dr. Psyklon)
		qúvadúvahosszú (Rendelés - Dr. Psyklon)
		R Vizsgálat (Rendelés - Dr. Psyklon)
		raul vizsg (Rendelés - Dr. Psyklon)
		sad (Rendelés - Dr. Psyklon)
		saddsad (3 - a)
		seggturkászat (Urológia - Dr. Brokvik Ulrich Hanburber Béla)
		sfa (Rendelés - Dr. Psyklon)
		spanyol csizma feladása (Teljestest átalakítás - Dr. Brokvik Ulrich Hanburber Béla)
		szemkivájás (Teljestest átalakítás - Dr. Brokvik Ulrich Hanburber Béla)
		Teszt beav 1 (Rendelés - Dr. Psyklon)
		Teszt beav 4 (Rendelés - Dr. Psyklon)
		Teszt beav 5 (Rendelés - Dr. Psyklon)
		Teszt beav 6 (Rendelés - Dr. Psyklon)
		Teszt beav 7 (Rendelés - Dr. Psyklon)
		Új beavatkozás (Rendelés - Dr. Psyklon)
		Utolsó próba (Rendelés - Dr. Psyklon)
		Zöld (Rendelés - Dr. Psyklon)
	
	
	
	



function _857eb31c01a1b5c98c737795c2d84b07_add() {
	if($('#email-sablon-beavatkozas').val() > 0) {
		$('#email-sablon-beavatkozas-labels').append('&lt;div class=&quot;labels-label&quot; data-id=&quot;'+$('#email-sablon-beavatkozas').val()+'&quot;>&lt;span>'+$('#email-sablon-beavatkozas option:selected').text()+'&lt;/span>&lt;div class=&quot;sm-remove&quot; onclick=&quot;_857eb31c01a1b5c98c737795c2d84b07_remove(this)&quot;>&lt;i class=&quot;fa fa-times&quot;>&lt;/i>&lt;/div>&lt;/div>')
		$('#email-sablon-beavatkozas option:selected').remove()
		_857eb31c01a1b5c98c737795c2d84b07_update()
	}
}
function _857eb31c01a1b5c98c737795c2d84b07_remove(elem) {
	$('#email-sablon-beavatkozas').append('&lt;option value=&quot;'+$(elem).parents('.labels-label').attr('data-id')+'&quot;>'+$(elem).parents('.labels-label').find('span').text()+'&lt;/option>')
	$(elem).parents('.labels-label').remove()
	_857eb31c01a1b5c98c737795c2d84b07_update()
}
function _857eb31c01a1b5c98c737795c2d84b07_update() {
	var ids = []
	$('#email-sablon-beavatkozas-labels .labels-label').each(function(){
		ids.push($(this).attr('data-id'))
	})
	$('input[name=&quot;email-sablon-beavatkozas&quot;]').val(ids.join(','))
	_857eb31c01a1b5c98c737795c2d84b07_resize()
}
function _857eb31c01a1b5c98c737795c2d84b07_reverse_update() {
	$('#email-sablon-beavatkozas-labels .labels-label').each(function(){
		$(this).find('.sm-remove').click()
	})
	var ids = $('input[name=&quot;email-sablon-beavatkozas&quot;]').val().split(',')
	for(var i = 0; i &lt; ids.length; i++) {
		if(ids[i] > 0) {
			$('#email-sablon-beavatkozas').val(ids[i])
			_857eb31c01a1b5c98c737795c2d84b07_add()
			console.log(ids[i])
		}
	}
}
function _857eb31c01a1b5c98c737795c2d84b07_resize() {
	$('#email-sablon-beavatkozas').css('height', $('#email-sablon-beavatkozas-labels').outerHeight() + 'px')
	$('#email-sablon-beavatkozas').parents('.form-ctrl').css('height', $('#email-sablon-beavatkozas-labels').outerHeight() + 'px')
}
components['email-sablon-beavatkozas']={&quot;label&quot;:&quot;Vizsg\u00e1latok&quot;,&quot;labelSize&quot;:&quot;no&quot;,&quot;value&quot;:&quot;&quot;,&quot;defaults&quot;:[&quot;\u00d6sszes hozz\u00e1ad\u00e1sa&quot;],&quot;options&quot;:[&quot;$client&quot;,&quot;listBeavatkozasLista&quot;,&quot;BeavatkozasLista&quot;,{&quot;dbName&quot;:&quot;B9019003&quot;},{&quot;BeavatkozasID&quot;:&quot;Nev&quot;}],&quot;_component&quot;:&quot;tags&quot;,&quot;_hashname&quot;:&quot;_857eb31c01a1b5c98c737795c2d84b07&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-beavatkozas&quot;};componentsLoaded['email-sablon-beavatkozas']={};Object.assign(componentsLoaded['email-sablon-beavatkozas'], components['email-sablon-beavatkozas']);




	Csatolmányok (PDF nyomtatvány)
	
		- - - - - - -
		asdasdmmmmm
		Cito kérő (csatolmány)
		program minőség,zu
	
	
	
	



function _c1265c58d8f639545d556a813f7046ab_add() {
	if($('#email-sablon-csatolmany').val() > 0) {
		$('#email-sablon-csatolmany-labels').append('&lt;div class=&quot;labels-label&quot; data-id=&quot;'+$('#email-sablon-csatolmany').val()+'&quot;>&lt;span>'+$('#email-sablon-csatolmany option:selected').text()+'&lt;/span>&lt;div class=&quot;sm-remove&quot; onclick=&quot;_c1265c58d8f639545d556a813f7046ab_remove(this)&quot;>&lt;i class=&quot;fa fa-times&quot;>&lt;/i>&lt;/div>&lt;/div>')
		$('#email-sablon-csatolmany option:selected').remove()
		_c1265c58d8f639545d556a813f7046ab_update()
	}
}
function _c1265c58d8f639545d556a813f7046ab_remove(elem) {
	$('#email-sablon-csatolmany').append('&lt;option value=&quot;'+$(elem).parents('.labels-label').attr('data-id')+'&quot;>'+$(elem).parents('.labels-label').find('span').text()+'&lt;/option>')
	$(elem).parents('.labels-label').remove()
	_c1265c58d8f639545d556a813f7046ab_update()
}
function _c1265c58d8f639545d556a813f7046ab_update() {
	var ids = []
	$('#email-sablon-csatolmany-labels .labels-label').each(function(){
		ids.push($(this).attr('data-id'))
	})
	$('input[name=&quot;email-sablon-csatolmany&quot;]').val(ids.join(','))
	_c1265c58d8f639545d556a813f7046ab_resize()
}
function _c1265c58d8f639545d556a813f7046ab_reverse_update() {
	$('#email-sablon-csatolmany-labels .labels-label').each(function(){
		$(this).find('.sm-remove').click()
	})
	var ids = $('input[name=&quot;email-sablon-csatolmany&quot;]').val().split(',')
	for(var i = 0; i &lt; ids.length; i++) {
		if(ids[i] > 0) {
			$('#email-sablon-csatolmany').val(ids[i])
			_c1265c58d8f639545d556a813f7046ab_add()
			console.log(ids[i])
		}
	}
}
function _c1265c58d8f639545d556a813f7046ab_resize() {
	$('#email-sablon-csatolmany').css('height', $('#email-sablon-csatolmany-labels').outerHeight() + 'px')
	$('#email-sablon-csatolmany').parents('.form-ctrl').css('height', $('#email-sablon-csatolmany-labels').outerHeight() + 'px')
}
components['email-sablon-csatolmany']={&quot;label&quot;:&quot;Csatolm\u00e1nyok (PDF nyomtatv\u00e1ny)&quot;,&quot;labelSize&quot;:&quot;no&quot;,&quot;value&quot;:&quot;&quot;,&quot;options&quot;:[&quot;$client&quot;,&quot;listPdfNyomtatvany&quot;,&quot;PdfNyomtatvanyok&quot;,{&quot;PdfNyomtatvanyTipusID&quot;:2,&quot;dbName&quot;:&quot;B9019003&quot;},{&quot;PdfNyomtatvanyID&quot;:&quot;Nev&quot;}],&quot;_component&quot;:&quot;tags&quot;,&quot;_hashname&quot;:&quot;_c1265c58d8f639545d556a813f7046ab&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-csatolmany&quot;};componentsLoaded['email-sablon-csatolmany']={};Object.assign(componentsLoaded['email-sablon-csatolmany'], components['email-sablon-csatolmany']);



	
	Sablon törlése


	function _1500cdbca00bdcd45824101ca9249b11_onclick(that){
deleteEmailSablon()
}

components['email-sablon-torles']={&quot;icon&quot;:&quot;trash&quot;,&quot;text&quot;:&quot;Sablon t\u00f6rl\u00e9se&quot;,&quot;color&quot;:&quot;green&quot;,&quot;disabled&quot;:true,&quot;onclick&quot;:&quot;deleteEmailSablon()&quot;,&quot;_component&quot;:&quot;button&quot;,&quot;_hashname&quot;:&quot;_1500cdbca00bdcd45824101ca9249b11&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-torles&quot;};componentsLoaded['email-sablon-torles']={};Object.assign(componentsLoaded['email-sablon-torles'], components['email-sablon-torles']);

	
	Mentés újként


	function _ba39ef2d8384bd6ecfd6271470fe8aab_onclick(that){
saveEmailSablon(-1)
}

components['email-sablon-mentes-uj']={&quot;icon&quot;:&quot;copy&quot;,&quot;text&quot;:&quot;Ment\u00e9s \u00fajk\u00e9nt&quot;,&quot;color&quot;:&quot;green&quot;,&quot;class&quot;:&quot;right mr-0&quot;,&quot;disabled&quot;:true,&quot;onclick&quot;:&quot;saveEmailSablon(-1)&quot;,&quot;_component&quot;:&quot;button&quot;,&quot;_hashname&quot;:&quot;_ba39ef2d8384bd6ecfd6271470fe8aab&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-mentes-uj&quot;};componentsLoaded['email-sablon-mentes-uj']={};Object.assign(componentsLoaded['email-sablon-mentes-uj'], components['email-sablon-mentes-uj']);

	
	Sablon mentése


	function _ce2c8f8061a4dffa8be43e31e9e63ccc_onclick(that){
saveEmailSablon()
}

components['email-sablon-mentes']={&quot;icon&quot;:&quot;save&quot;,&quot;text&quot;:&quot;Sablon ment\u00e9se&quot;,&quot;color&quot;:&quot;green&quot;,&quot;class&quot;:&quot;right&quot;,&quot;disabled&quot;:true,&quot;onclick&quot;:&quot;saveEmailSablon()&quot;,&quot;_component&quot;:&quot;button&quot;,&quot;_hashname&quot;:&quot;_ce2c8f8061a4dffa8be43e31e9e63ccc&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-sablon-mentes&quot;};componentsLoaded['email-sablon-mentes']={};Object.assign(componentsLoaded['email-sablon-mentes'], components['email-sablon-mentes']);


components['email-modal']={&quot;contents&quot;:[&quot;&lt;div data-component=\&quot;select\&quot; data-name=\&quot;email-sablon-select\&quot; data-hash=\&quot;2bba2769065ec7426c392f58fe08da4e\&quot;>&lt;div class=\&quot;form-ctrl\&quot;>\r\n&lt;label>E-mail sablon&lt;\/label>\r\n&lt;select id=\&quot;email-sablon-select\&quot; onchange=\&quot;changeEmailSablon(this)\&quot; class=\&quot;small-label\&quot;>\r\n\t&lt;option value=\&quot;-1\&quot;>&amp;lt;\u00daj sablon&amp;gt;&lt;\/option>\r\n\t&lt;option value=\&quot;31\&quot; data-name=\&quot;100 Folk\&quot; data-type=\&quot;vizsgalatonkent\&quot; data-subject=\&quot;100 Folk Celsius\&quot; data-text=\&quot;&lt;div>K\u00f6sz\u00f6nj\u00fck hogy r\u00e9szt vesz a 100 Folk Celsius koncerten!&lt;\/div>\&quot; data-beav=\&quot;61\&quot;>100 Folk&lt;\/option>\r\n\t&lt;option value=\&quot;30\&quot; data-name=\&quot;asdasdasdasdadasdasw\&quot; data-type=\&quot;altalanos\&quot; data-subject=\&quot;asdasdasdasdasdasdw\&quot; data-text=\&quot;&lt;div>asdasdasdasdasdasdwasdasdasdasdasdasdwasdasdasdasdasdasdwasdasdasdasdasdasdwasdasdasdasdasdasdwasdasdasdasdasdasdw&lt;\/div>\&quot; data-beav=\&quot;\&quot;>asdasdasdasdadasdasw&lt;\/option>\r\n\t&lt;option value=\&quot;16\&quot; data-name=\&quot;Citol\u00f3giai email csatolm\u00e1ny P1\&quot; data-type=\&quot;cito_csatolmany\&quot; data-subject=\&quot;P1\&quot; data-text=\&quot;&lt;div>Az \u00d6n eredm\u00e9nye: #eredmeny#.zzzz&lt;\/div>\&quot; data-beav=\&quot;\&quot;>Citol\u00f3giai email csatolm\u00e1ny P1&lt;\/option>\r\n\t&lt;option value=\&quot;21\&quot; data-name=\&quot;Citol\u00f3giai email csatolm\u00e1ny P2\&quot; data-type=\&quot;cito_csatolmany\&quot; data-subject=\&quot;P2\&quot; data-text=\&quot;&lt;div>Az \u00d6n eredm\u00e9nye: #eredmeny#.&lt;\/div>\&quot; data-beav=\&quot;\&quot;>Citol\u00f3giai email csatolm\u00e1ny P2&lt;\/option>\r\n\t&lt;option value=\&quot;15\&quot; data-name=\&quot;Citol\u00f3giai ki\u00e9rtes\u00edt\u00e9s\&quot; data-type=\&quot;cito_email\&quot; data-subject=\&quot;Citol\u00f3giai eredm\u00e9ny\&quot; data-text=\&quot;&lt;div>\n&lt;div>Tisztelt P&amp;aacute;ciens&amp;uuml;nk!&lt;\/div>\n&lt;div>&amp;nbsp;&lt;\/div>\n&lt;div>Csatoltan k&amp;uuml;ldj&amp;uuml;k &amp;Ouml;nnek citol&amp;oacute;giai vizsg&amp;aacute;lat&amp;aacute;nak eredm&amp;eacute;ny&amp;eacute;t, mely csatolm&amp;aacute;nyt TAJ sz&amp;aacute;m&amp;aacute;val vagy annak hi&amp;aacute;ny&amp;aacute;ban (&amp;Eacute;&amp;Eacute;&amp;Eacute;&amp;Eacute;.HH.NN form&amp;aacute;tum&amp;uacute;) sz&amp;uuml;let&amp;eacute;si d&amp;aacute;tum&amp;aacute;val tudja megnyitni.&lt;\/div>\n&lt;\/div>\&quot; data-beav=\&quot;\&quot;>Citol\u00f3giai ki\u00e9rtes\u00edt\u00e9s&lt;\/option>\r\n\t&lt;option value=\&quot;32\&quot; data-name=\&quot;Csatolm\u00e1nyos\&quot; data-type=\&quot;altalanos\&quot; data-subject=\&quot;Csatolm\u00e1nyos\&quot; data-text=\&quot;&lt;div>Csatolm\u00e1nyos&lt;\/div>\&quot; data-beav=\&quot;\&quot;>Csatolm\u00e1nyos&lt;\/option>\r\n\t&lt;option value=\&quot;26\&quot; data-name=\&quot;Cs\u00f6vi vizsg\u00e1lat ki\u00e9rtes\u00edt\u00e9s\&quot; data-type=\&quot;vizsgalatonkent\&quot; data-subject=\&quot;Cs\u00f6ves\&quot; data-text=\&quot;&lt;div>abc&lt;\/div>\&quot; data-beav=\&quot;22\&quot;>Cs\u00f6vi vizsg\u00e1lat ki\u00e9rtes\u00edt\u00e9s&lt;\/option>\r\n\t&lt;option value=\&quot;22\&quot; data-name=\&quot;ed\&quot; data-type=\&quot;altalanos\&quot; data-subject=\&quot;ed\&quot; data-text=\&quot;&lt;div>&amp;nbsp;()..()&lt;\/div>&lt;div>( o.o )&lt;\/div>&lt;div>(&amp;quot;)_(&amp;quot;)&lt;\/div>\&quot; data-beav=\&quot;\&quot;>ed&lt;\/option>\r\n\t&lt;option value=\&quot;2\&quot; data-name=\&quot;El\u0151jegyz\u00e9s online\&quot; data-type=\&quot;globalis\&quot; data-subject=\&quot;Kardi-Soft : Online el\u0151jegyz\u00e9s\u00e9t r\u00f6gz\u00edtett\u00fck\&quot; data-text=\&quot;&lt;div>Tisztelt&amp;nbsp;#ElojegyzesNev#!&lt;\/div>&lt;div>&lt;br>&lt;\/div>&lt;div>El\u0151jegyz\u00e9s\u00e9t r\u00f6gz\u00edtett\u00fck al\u00e1bbi munkat\u00e1rsunkhoz:&lt;\/div>&lt;div>&lt;strong>Orvos:&lt;\/strong>&amp;nbsp;#ElojegyzesOrvos#&lt;br>&lt;strong>Szakrendel\u00e9s:&lt;\/strong>&amp;nbsp;#ElojegyzesSzakrendeles#&lt;\/div>&lt;div>&lt;br>&lt;\/div>&lt;div>A k\u00f6vetkez\u0151 adatokkal:&lt;br>&lt;strong>Vizsg\u00e1lat:&lt;\/strong>&amp;nbsp;#ElojegyzesVizsgalat#&lt;br>&lt;strong>D\u00e1tum:&lt;\/strong> #ElojegyzesDatum#&lt;br>&lt;strong>Id\u0151pont:&lt;\/strong>&amp;nbsp;#ElojegyzesIdopont#&lt;br>&lt;strong>N\u00e9v:&lt;\/strong> #ElojegyzesNev#&lt;br>&lt;strong>TAJ:&lt;\/strong>&amp;nbsp;#ElojegyzesTAJ#&lt;br>&lt;strong>Sz\u00fclet\u00e9si d\u00e1tum:&lt;\/strong>&amp;nbsp;#ElojegyzesSzuletesiDatum#&lt;br>&lt;strong>E-mail c\u00edm:&lt;\/strong>&amp;nbsp;#ElojegyzesEmail#&lt;br>&lt;strong>Telefon:&lt;\/strong>&amp;nbsp;#ElojegyzesTelefon#&lt;br>&lt;strong>Megjegyz\u00e9s:&lt;\/strong>&amp;nbsp;#ElojegyzesMegjegyzes#&lt;\/div>&lt;div>&lt;br>&lt;\/div>&lt;div>Amennyiben m\u00e9gsem tudna elj\u00f6nni vagy az el\u0151jegyz\u00e9st v\u00e9letlen\u00fcl adta le, \u00fagy az al\u00e1bbi linken tudja lemondani:&lt;\/div>&lt;div>#ElojegyzesTorlesURL#&lt;\/div>&lt;div>&lt;br>&lt;\/div>&lt;div>K\u00f6sz\u00f6nj\u00fck hogy minket v\u00e1lasztott!&lt;\/div>&lt;div>&lt;br>&lt;\/div>&lt;div>\u00dcdv\u00f6zlettel,&lt;\/div>&lt;div>#rendeloNev#&lt;\/div>&lt;div>#rendeloCim#&lt;\/div>&lt;div>#rendeloTel#&lt;\/div>\&quot; data-beav=\&quot;\&quot;>El\u0151jegyz\u00e9s online&lt;\/option>\r\n\t&lt;option value=\&quot;25\&quot; data-name=\&quot;Email sablon vizsg\u00e1lathoz\&quot; data-type=\&quot;vizsgalatonkent\&quot; data-subject=\&quot;Ja\&quot; data-text=\&quot;&lt;div>sd&lt;\/div>\&quot; data-beav=\&quot;43,24,19\&quot;>Email sablon vizsg\u00e1lathoz&lt;\/option>\r\n\t&lt;option value=\&quot;29\&quot; data-name=\&quot;kruzg\&quot; data-type=\&quot;vizsgalatonkent\&quot; data-subject=\&quot;mtdgu\&quot; data-text=\&quot;&lt;div>kk&lt;\/div>\&quot; data-beav=\&quot;34,31,15,19,13,27,39,11,22,33,25,42,35\&quot;>kruzg&lt;\/option>\r\n\t&lt;option value=\&quot;6\&quot; data-name=\&quot;N\u00e9mileg \u00faj \u00e1ltal\u00e1nos sablon\&quot; data-type=\&quot;altalanos\&quot; data-subject=\&quot;\u00dczenetet k\u00fcld\u00fcnk ha kell, ha nem!\&quot; data-text=\&quot;&lt;div>\u00dcdv\u00f6zl\u00f6m kedves #PaciensNev#!&lt;\/div>&lt;div>&lt;br data-mce-bogus=&amp;quot;1&amp;quot;>&lt;\/div>&lt;div>Foglaljon el\u0151 itten: #PaciensElojegyzesURL#&lt;\/div>&lt;div>&lt;br data-mce-bogus=&amp;quot;1&amp;quot;>&lt;\/div>\&quot; data-beav=\&quot;\&quot;>N\u00e9mileg \u00faj \u00e1ltal\u00e1nos sablon&lt;\/option>\r\n\t&lt;option value=\&quot;27\&quot; data-name=\&quot;nogyogy_negativ\&quot; data-type=\&quot;cito_csatolmany\&quot; data-subject=\&quot;nogyogy_negativ\&quot; data-text=\&quot;&lt;div>Kedves P\u00e1ciens\u00fcnk!&lt;\/div>&lt;div>A vizsg\u00e1lat eredm\u00e9nye:&amp;nbsp; #eredmeny#&amp;nbsp; !&lt;\/div>&lt;div>\u00dcdv.:&lt;\/div>&lt;div>doki&lt;\/div>&lt;div>&lt;br data-mce-bogus=&amp;quot;1&amp;quot;>&lt;\/div>\&quot; data-beav=\&quot;\&quot;>nogyogy_negativ&lt;\/option>\r\n\t&lt;option value=\&quot;23\&quot; data-name=\&quot;Nyulak\&quot; data-type=\&quot;altalanos\&quot; data-subject=\&quot;Nyulak\&quot; data-text=\&quot;&lt;div>&amp;nbsp;()..()&lt;\/div>&lt;div>(o.o )&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; ()..()&lt;\/div>&lt;div>(&amp;quot;)_(&amp;quot;)&amp;nbsp; &amp;nbsp; &amp;nbsp; ( o.o)&lt;\/div>&lt;div>&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp;(&amp;quot;)_(&amp;quot;)&lt;\/div>&lt;div>&lt;br data-mce-bogus=&amp;quot;1&amp;quot;>&lt;\/div>&lt;div>&lt;div>&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp;()..()&lt;\/div>&lt;div>&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; ( o.o )&lt;\/div>&lt;div>&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; (&amp;quot;)_(&amp;quot;)&lt;\/div>&lt;div>&lt;br data-mce-bogus=&amp;quot;1&amp;quot;>&lt;\/div>&lt;\/div>\&quot; data-beav=\&quot;\&quot;>Nyulak&lt;\/option>\r\n\t&lt;option value=\&quot;24\&quot; data-name=\&quot;Nyulak 2\&quot; data-type=\&quot;altalanos\&quot; data-subject=\&quot;Nyulak\&quot; data-text=\&quot;&lt;div>&amp;nbsp;()..()&lt;\/div>&lt;div>( o.o)&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; ()..()&lt;\/div>&lt;div>(&amp;quot;)_(&amp;quot;)&amp;nbsp; &amp;nbsp; &amp;nbsp; (o.o )&lt;\/div>&lt;div>&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp;(&amp;quot;)_(&amp;quot;)&lt;\/div>&lt;div>&lt;br>&lt;\/div>&lt;div>&lt;div>&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp;()..()&lt;\/div>&lt;div>&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; ( o.o )&lt;\/div>&lt;div>&amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; &amp;nbsp; (&amp;quot;)_(&amp;quot;)&lt;\/div>&lt;div>&lt;br>&lt;\/div>&lt;\/div>\&quot; data-beav=\&quot;\&quot;>Nyulak 2&lt;\/option>\r\n\t&lt;option value=\&quot;28\&quot; data-name=\&quot;oujlk\&quot; data-type=\&quot;altalanos\&quot; data-subject=\&quot;OJ\&quot; data-text=\&quot;&lt;div>jJUOU&lt;\/div>\&quot; data-beav=\&quot;\&quot;>oujlk&lt;\/option>\r\n\t&lt;option value=\&quot;17\&quot; data-name=\&quot;\u00daJ TESZT 2\&quot; data-type=\&quot;altalanos\&quot; data-subject=\&quot;T\u00c1RGY\&quot; data-text=\&quot;&lt;div>lorem ipsum&lt;\/div>\&quot; data-beav=\&quot;\&quot;>\u00daJ TESZT 2&lt;\/option>\r\n&lt;\/select>\r\n&lt;\/div>\r\n&lt;script data-eval>components['email-sablon-select']={\&quot;label\&quot;:\&quot;E-mail sablon\&quot;,\&quot;class\&quot;:\&quot;small-label\&quot;,\&quot;defaults\&quot;:{\&quot;-1\&quot;:\&quot;&lt;\\u00daj sablon>\&quot;},\&quot;options\&quot;:[\&quot;$client\&quot;,\&quot;listEmailSablon\&quot;,\&quot;EmailSablonok\&quot;,{\&quot;dbName\&quot;:\&quot;B9019003\&quot;},\&quot;EmailSablonID\&quot;],\&quot;format\&quot;:true,\&quot;name\&quot;:\&quot;{Nev}\&quot;,\&quot;metadata\&quot;:{\&quot;data-name\&quot;:\&quot;$Nev\&quot;,\&quot;data-type\&quot;:\&quot;$Tipus\&quot;,\&quot;data-subject\&quot;:\&quot;$Targy\&quot;,\&quot;data-text\&quot;:\&quot;$Szoveg\&quot;,\&quot;data-beav\&quot;:\&quot;$BeavatkozasID\&quot;},\&quot;onchange\&quot;:\&quot;changeEmailSablon(this)\&quot;,\&quot;_component\&quot;:\&quot;select\&quot;,\&quot;_hashname\&quot;:\&quot;_0e5dedd730b9c216da8e7c1f93ebea58\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-select\&quot;};componentsLoaded['email-sablon-select']={};Object.assign(componentsLoaded['email-sablon-select'], components['email-sablon-select']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;div data-component=\&quot;inputs.text\&quot; data-name=\&quot;email-sablon-nev\&quot; data-hash=\&quot;7aa49183703a6acdee1797f52bc286b5\&quot;>&lt;div class=\&quot;form-ctrl\&quot;>\r\n\t&lt;label for=\&quot;email-sablon-nev\&quot;>N\u00e9v\r\n\t&lt;\/label>\r\n\t&lt;input type=\&quot;text\&quot; name=\&quot;email-sablon-nev\&quot; value=\&quot;\&quot; class=\&quot;small-label\&quot; required oninput=\&quot;checkEmailSablonFields()\&quot;>\r\n&lt;\/div>\r\n&lt;script data-eval>components['email-sablon-nev']={\&quot;name\&quot;:\&quot;email-sablon-nev\&quot;,\&quot;input\&quot;:{\&quot;label\&quot;:\&quot;N\\u00e9v\&quot;,\&quot;labelSize\&quot;:\&quot;small\&quot;,\&quot;required\&quot;:true,\&quot;onchange\&quot;:\&quot;checkEmailSablonFields()\&quot;},\&quot;_component\&quot;:\&quot;inputs.text\&quot;,\&quot;_hashname\&quot;:\&quot;_b082dba326042adbb66f8d079ad70ded\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-nev\&quot;};componentsLoaded['email-sablon-nev']={};Object.assign(componentsLoaded['email-sablon-nev'], components['email-sablon-nev']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;div data-component=\&quot;inputs.text\&quot; data-name=\&quot;email-sablon-targy\&quot; data-hash=\&quot;46c85eafe9572a7f0881d599dc16353d\&quot;>&lt;div class=\&quot;form-ctrl\&quot;>\r\n\t&lt;label for=\&quot;email-sablon-targy\&quot;>T\u00e1rgy\r\n\t&lt;\/label>\r\n\t&lt;input type=\&quot;text\&quot; name=\&quot;email-sablon-targy\&quot; value=\&quot;\&quot; class=\&quot;small-label\&quot; required oninput=\&quot;checkEmailSablonFields()\&quot;>\r\n&lt;\/div>\r\n&lt;script data-eval>components['email-sablon-targy']={\&quot;name\&quot;:\&quot;email-sablon-targy\&quot;,\&quot;input\&quot;:{\&quot;label\&quot;:\&quot;T\\u00e1rgy\&quot;,\&quot;labelSize\&quot;:\&quot;small\&quot;,\&quot;required\&quot;:true,\&quot;onchange\&quot;:\&quot;checkEmailSablonFields()\&quot;},\&quot;_component\&quot;:\&quot;inputs.text\&quot;,\&quot;_hashname\&quot;:\&quot;_e1a3d1ddd40b9341f33443029131dbe9\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-targy\&quot;};componentsLoaded['email-sablon-targy']={};Object.assign(componentsLoaded['email-sablon-targy'], components['email-sablon-targy']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;div class=\&quot;row\&quot; style=\&quot;margin-bottom:12px\&quot;>&quot;,&quot;&lt;div data-component=\&quot;inputs.textarea\&quot; data-name=\&quot;email-sablon-szoveg\&quot; data-hash=\&quot;0930e31fc37554d0b0a8e34f41a3c589\&quot;>&lt;textarea name=\&quot;email-sablon-szoveg\&quot; id=\&quot;editor\&quot; class=\&quot;tinymce\&quot;>&lt;\/textarea>\r\n&lt;script data-eval>components['email-sablon-szoveg']={\&quot;name\&quot;:\&quot;email-sablon-szoveg\&quot;,\&quot;input\&quot;:{\&quot;id\&quot;:\&quot;editor\&quot;,\&quot;class\&quot;:\&quot;tinymce\&quot;,\&quot;nowrap\&quot;:\&quot;true\&quot;},\&quot;_component\&quot;:\&quot;inputs.textarea\&quot;,\&quot;_hashname\&quot;:\&quot;_a02cf9733718c2aed202a7fd0e2f7c8b\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-szoveg\&quot;};componentsLoaded['email-sablon-szoveg']={};Object.assign(componentsLoaded['email-sablon-szoveg'], components['email-sablon-szoveg']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;\/div>&quot;,&quot;&lt;div data-component=\&quot;select\&quot; data-name=\&quot;email-sablon-tipus\&quot; data-hash=\&quot;a8b313c25822f878f9b7e81c9fc7ce85\&quot;>&lt;div class=\&quot;form-ctrl\&quot;>\r\n&lt;label>Sablon t\u00edpusa&lt;\/label>\r\n&lt;select id=\&quot;email-sablon-tipus\&quot; onchange=\&quot;checkSablonTipus()\&quot; class=\&quot;small-label\&quot;>\r\n\t&lt;option value=\&quot;altalanos\&quot;>\u00c1ltal\u00e1nos&lt;\/option>\r\n\t&lt;option value=\&quot;globalis\&quot;>Visszaigazol\u00e1s el\u0151jegyz\u00e9sn\u00e9l (glob\u00e1lis)&lt;\/option>\r\n\t&lt;option value=\&quot;vizsgalatonkent\&quot;>Visszaigazol\u00e1s el\u0151jegyz\u00e9sn\u00e9l (vizsg\u00e1latonk\u00e9nt)&lt;\/option>\r\n\t&lt;option value=\&quot;cito_email\&quot;>Citol\u00f3giai \u00e9rtes\u00edt\u0151&lt;\/option>\r\n\t&lt;option value=\&quot;cito_csatolmany\&quot;>Citol\u00f3giai \u00e9rtes\u00edt\u0151 csatolm\u00e1nya&lt;\/option>\r\n&lt;\/select>\r\n&lt;\/div>\r\n&lt;script data-eval>components['email-sablon-tipus']={\&quot;label\&quot;:\&quot;Sablon t\\u00edpusa\&quot;,\&quot;class\&quot;:\&quot;small-label\&quot;,\&quot;onchange\&quot;:\&quot;checkSablonTipus()\&quot;,\&quot;options\&quot;:{\&quot;altalanos\&quot;:\&quot;\\u00c1ltal\\u00e1nos\&quot;,\&quot;globalis\&quot;:\&quot;Visszaigazol\\u00e1s el\\u0151jegyz\\u00e9sn\\u00e9l (glob\\u00e1lis)\&quot;,\&quot;vizsgalatonkent\&quot;:\&quot;Visszaigazol\\u00e1s el\\u0151jegyz\\u00e9sn\\u00e9l (vizsg\\u00e1latonk\\u00e9nt)\&quot;,\&quot;cito_email\&quot;:\&quot;Citol\\u00f3giai \\u00e9rtes\\u00edt\\u0151\&quot;,\&quot;cito_csatolmany\&quot;:\&quot;Citol\\u00f3giai \\u00e9rtes\\u00edt\\u0151 csatolm\\u00e1nya\&quot;},\&quot;_component\&quot;:\&quot;select\&quot;,\&quot;_hashname\&quot;:\&quot;_1f433fdc5eabbb616242ee8abde980a5\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-tipus\&quot;};componentsLoaded['email-sablon-tipus']={};Object.assign(componentsLoaded['email-sablon-tipus'], components['email-sablon-tipus']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;div id=\&quot;beav-select\&quot; class=\&quot;row auto-height hidden\&quot;>&quot;,&quot;&lt;div data-component=\&quot;tags\&quot; data-name=\&quot;email-sablon-beavatkozas\&quot; data-hash=\&quot;60a905bc6bed2b1b23365a5667ab9353\&quot;>&lt;div>\r\n&lt;div class=\&quot;form-ctrl labels-container\&quot;>\r\n\t&lt;label>Vizsg\u00e1latok&lt;\/label>\r\n\t&lt;select id=\&quot;email-sablon-beavatkozas\&quot; onchange=\&quot;_857eb31c01a1b5c98c737795c2d84b07_add()\&quot; class=\&quot;no-label\&quot;>\r\n\t\t&lt;option value=\&quot;\&quot;>- - - - - - -&lt;\/option>\r\n\t\t&lt;option value=\&quot;0\&quot;>\u00d6sszes hozz\u00e1ad\u00e1sa&lt;\/option>\r\n\t\t&lt;option value=\&quot;61\&quot;>100 folk celsius koncert (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;37\&quot;>100 minute (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;62\&quot;>20 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;64\&quot;>20 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;60\&quot;>3892zr6\u00f6 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;84\&quot;>3rf34f (3 - a)&lt;\/option>\r\n\t\t&lt;option value=\&quot;29\&quot;>5 perces (3 - TTTTTTTTTTTTTTT)&lt;\/option>\r\n\t\t&lt;option value=\&quot;89\&quot;>55 perces (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;93\&quot;>5min (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;94\&quot;>5p (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;51\&quot;>a (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;88\&quot;>a r\u00e9gi (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;28\&quot;>agy\u00e1t\u00fcltet\u00e9s (3 - TTTTTTTTTTTTTTT)&lt;\/option>\r\n\t\t&lt;option value=\&quot;31\&quot;>agy\u00e1t\u00fcltet\u00e9s (N\u0151gy\u00f3gy\u00e1szat - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;17\&quot;>agy\u00e1t\u00fcltet\u00e9s (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;44\&quot;>agyzsugor\u00edt\u00e1s (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;25\&quot;>alt (Seb\u00e9szet - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;77\&quot;>asd (3 - a)&lt;\/option>\r\n\t\t&lt;option value=\&quot;68\&quot;>asd (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;52\&quot;>asd (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;63\&quot;>asd (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;90\&quot;>asd (Teljestest \u00e1talak\u00edt\u00e1s - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;83\&quot;>asdadsdsa (3 - a)&lt;\/option>\r\n\t\t&lt;option value=\&quot;82\&quot;>asddasds (3 - a)&lt;\/option>\r\n\t\t&lt;option value=\&quot;70\&quot;>Asdeff (3 - Dr. Teszt Elek)&lt;\/option>\r\n\t\t&lt;option value=\&quot;79\&quot;>asdsadsa (3 - a)&lt;\/option>\r\n\t\t&lt;option value=\&quot;81\&quot;>asdsdsa (3 - a)&lt;\/option>\r\n\t\t&lt;option value=\&quot;47\&quot;>asfsa (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;56\&quot;>b (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;19\&quot;>By Fire Be Purged! (Rendel\u00e9s - Ragnaros)&lt;\/option>\r\n\t\t&lt;option value=\&quot;55\&quot;>c (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;22\&quot;>cs\u00f6vi vizsg\u00e1lat (Avar utca - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;33\&quot;>cs\u00f6vi vizsg\u00e1lat (Avar utca - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;34\&quot;>cs\u00f6vi vizsg\u00e1lat (Avar utca - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;35\&quot;>cs\u00f6vi vizsg\u00e1lat (Avar utca - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;36\&quot;>cs\u00f6vi vizsg\u00e1lat (Avar utca - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;45\&quot;>cs\u00f6vi vizsg\u00e1lat (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;54\&quot;>d (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;50\&quot;>dasdas (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;78\&quot;>def (3 - a)&lt;\/option>\r\n\t\t&lt;option value=\&quot;26\&quot;>delete (Seb\u00e9szet - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;27\&quot;>EB\u00c9DSZ\u00dcNET (Seb\u00e9szet - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;40\&quot;>els\u0151 vizit (N\u0151gy\u00f3gy\u00e1szat - Dr. Bek\u00e9ne Antal)&lt;\/option>\r\n\t\t&lt;option value=\&quot;85\&quot;>\u00e9rv\u00e1g\u00e1s (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;71\&quot;>fikat\u00f6rl\u00e9s (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;87\&quot;>f\u00fcllet\u00e9p\u00e9s (Teljestest \u00e1talak\u00edt\u00e1s - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;21\&quot;>f\u00fclmos\u00e1s (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;59\&quot;>Gy\u00f3gytorna (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;42\&quot;>here\u00e1t\u00fcltet\u00e9s (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;74\&quot;>kar\u00f3ba h\u00faz\u00e1s (Teljestest \u00e1talak\u00edt\u00e1s - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;57\&quot;>K\u00e9k (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;48\&quot;>K\u00e9k vizsg\u00e1lat 20 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;75\&quot;>ker\u00e9kbe t\u00f6r\u00e9s (Teljestest \u00e1talak\u00edt\u00e1s - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;72\&quot;>k\u00e9z-l\u00e1b lev\u00e9tel (Teljestest \u00e1talak\u00edt\u00e1s - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;92\&quot;>k\u00eds\u00e9rlet 1 (N\u0151gy\u00f3gy\u00e1szat - Dr. Josef Mengele)&lt;\/option>\r\n\t\t&lt;option value=\&quot;41\&quot;>kontroll (N\u0151gy\u00f3gy\u00e1szat - Dr. Bek\u00e9ne Antal)&lt;\/option>\r\n\t\t&lt;option value=\&quot;32\&quot;>kontroll (N\u0151gy\u00f3gy\u00e1szat - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;11\&quot;>kontroll (Seb\u00e9szet - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;73\&quot;>lefejez\u00e9s (Teljestest \u00e1talak\u00edt\u00e1s - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;14\&quot;>M\u00e1sik pr\u00f3ba (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;49\&quot;>pil\u00e1teszes karate (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;53\&quot;>Pil\u00e1teszes Teszt (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;39\&quot;>p\u00f6csmetsz\u00e9s (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;20\&quot;>Pr\u00f3ba (N\u0151gy\u00f3gy\u00e1szat - Teszt Elek)&lt;\/option>\r\n\t\t&lt;option value=\&quot;13\&quot;>Pr\u00f3ba (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;30\&quot;>q\u00favad\u00favahossz\u00fa (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;91\&quot;>R Vizsg\u00e1lat (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;66\&quot;>raul vizsg (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;65\&quot;>sad (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;80\&quot;>saddsad (3 - a)&lt;\/option>\r\n\t\t&lt;option value=\&quot;38\&quot;>seggturk\u00e1szat (Urol\u00f3gia - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;46\&quot;>sfa (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;76\&quot;>spanyol csizma felad\u00e1sa (Teljestest \u00e1talak\u00edt\u00e1s - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;86\&quot;>szemkiv\u00e1j\u00e1s (Teljestest \u00e1talak\u00edt\u00e1s - Dr. Brokvik Ulrich Hanburber B\u00e9la)&lt;\/option>\r\n\t\t&lt;option value=\&quot;3\&quot;>Teszt beav 1 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;6\&quot;>Teszt beav 4 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;2\&quot;>Teszt beav 5 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;1\&quot;>Teszt beav 6 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;7\&quot;>Teszt beav 7 (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;12\&quot;>\u00daj beavatkoz\u00e1s (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;15\&quot;>Utols\u00f3 pr\u00f3ba (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t\t&lt;option value=\&quot;58\&quot;>Z\u00f6ld (Rendel\u00e9s - Dr. Psyklon)&lt;\/option>\r\n\t&lt;\/select>\r\n\t&lt;div id=\&quot;email-sablon-beavatkozas-labels\&quot; class=\&quot;labels-input\&quot;>\r\n\t&lt;\/div>\r\n\t&lt;input type=\&quot;hidden\&quot; name=\&quot;email-sablon-beavatkozas\&quot; value=\&quot;\&quot; onchange=\&quot;_857eb31c01a1b5c98c737795c2d84b07_reverse_update()\&quot;>\r\n&lt;\/div>\r\n&lt;\/div>\r\n&lt;script data-eval>\r\nfunction _857eb31c01a1b5c98c737795c2d84b07_add() {\r\n\tif($('#email-sablon-beavatkozas').val() > 0) {\r\n\t\t$('#email-sablon-beavatkozas-labels').append('&lt;div class=\&quot;labels-label\&quot; data-id=\&quot;'+$('#email-sablon-beavatkozas').val()+'\&quot;>&lt;span>'+$('#email-sablon-beavatkozas option:selected').text()+'&lt;\/span>&lt;div class=\&quot;sm-remove\&quot; onclick=\&quot;_857eb31c01a1b5c98c737795c2d84b07_remove(this)\&quot;>&lt;i class=\&quot;fa fa-times\&quot;>&lt;\/i>&lt;\/div>&lt;\/div>')\r\n\t\t$('#email-sablon-beavatkozas option:selected').remove()\r\n\t\t_857eb31c01a1b5c98c737795c2d84b07_update()\r\n\t}\r\n}\r\nfunction _857eb31c01a1b5c98c737795c2d84b07_remove(elem) {\r\n\t$('#email-sablon-beavatkozas').append('&lt;option value=\&quot;'+$(elem).parents('.labels-label').attr('data-id')+'\&quot;>'+$(elem).parents('.labels-label').find('span').text()+'&lt;\/option>')\r\n\t$(elem).parents('.labels-label').remove()\r\n\t_857eb31c01a1b5c98c737795c2d84b07_update()\r\n}\r\nfunction _857eb31c01a1b5c98c737795c2d84b07_update() {\r\n\tvar ids = []\r\n\t$('#email-sablon-beavatkozas-labels .labels-label').each(function(){\r\n\t\tids.push($(this).attr('data-id'))\r\n\t})\r\n\t$('input[name=\&quot;email-sablon-beavatkozas\&quot;]').val(ids.join(','))\r\n\t_857eb31c01a1b5c98c737795c2d84b07_resize()\r\n}\r\nfunction _857eb31c01a1b5c98c737795c2d84b07_reverse_update() {\r\n\t$('#email-sablon-beavatkozas-labels .labels-label').each(function(){\r\n\t\t$(this).find('.sm-remove').click()\r\n\t})\r\n\tvar ids = $('input[name=\&quot;email-sablon-beavatkozas\&quot;]').val().split(',')\r\n\tfor(var i = 0; i &lt; ids.length; i++) {\r\n\t\tif(ids[i] > 0) {\r\n\t\t\t$('#email-sablon-beavatkozas').val(ids[i])\r\n\t\t\t_857eb31c01a1b5c98c737795c2d84b07_add()\r\n\t\t\tconsole.log(ids[i])\r\n\t\t}\r\n\t}\r\n}\r\nfunction _857eb31c01a1b5c98c737795c2d84b07_resize() {\r\n\t$('#email-sablon-beavatkozas').css('height', $('#email-sablon-beavatkozas-labels').outerHeight() + 'px')\r\n\t$('#email-sablon-beavatkozas').parents('.form-ctrl').css('height', $('#email-sablon-beavatkozas-labels').outerHeight() + 'px')\r\n}\r\n&lt;\/script>&lt;script data-eval>components['email-sablon-beavatkozas']={\&quot;label\&quot;:\&quot;Vizsg\\u00e1latok\&quot;,\&quot;labelSize\&quot;:\&quot;no\&quot;,\&quot;value\&quot;:\&quot;\&quot;,\&quot;defaults\&quot;:[\&quot;\\u00d6sszes hozz\\u00e1ad\\u00e1sa\&quot;],\&quot;options\&quot;:[\&quot;$client\&quot;,\&quot;listBeavatkozasLista\&quot;,\&quot;BeavatkozasLista\&quot;,{\&quot;dbName\&quot;:\&quot;B9019003\&quot;},{\&quot;BeavatkozasID\&quot;:\&quot;Nev\&quot;}],\&quot;_component\&quot;:\&quot;tags\&quot;,\&quot;_hashname\&quot;:\&quot;_857eb31c01a1b5c98c737795c2d84b07\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-beavatkozas\&quot;};componentsLoaded['email-sablon-beavatkozas']={};Object.assign(componentsLoaded['email-sablon-beavatkozas'], components['email-sablon-beavatkozas']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;\/div>&quot;,&quot;&lt;div id=\&quot;csat-select\&quot; class=\&quot;row auto-height hidden\&quot;>&quot;,&quot;&lt;div data-component=\&quot;tags\&quot; data-name=\&quot;email-sablon-csatolmany\&quot; data-hash=\&quot;1d9d4c0efe9102f67fc3a01be01cb2a3\&quot;>&lt;div>\r\n&lt;div class=\&quot;form-ctrl labels-container\&quot;>\r\n\t&lt;label>Csatolm\u00e1nyok (PDF nyomtatv\u00e1ny)&lt;\/label>\r\n\t&lt;select id=\&quot;email-sablon-csatolmany\&quot; onchange=\&quot;_c1265c58d8f639545d556a813f7046ab_add()\&quot; class=\&quot;no-label\&quot;>\r\n\t\t&lt;option value=\&quot;\&quot;>- - - - - - -&lt;\/option>\r\n\t\t&lt;option value=\&quot;17\&quot;>asdasdmmmmm&lt;\/option>\r\n\t\t&lt;option value=\&quot;18\&quot;>Cito k\u00e9r\u0151 (csatolm\u00e1ny)&lt;\/option>\r\n\t\t&lt;option value=\&quot;19\&quot;>program min\u0151s\u00e9g,zu&lt;\/option>\r\n\t&lt;\/select>\r\n\t&lt;div id=\&quot;email-sablon-csatolmany-labels\&quot; class=\&quot;labels-input\&quot;>\r\n\t&lt;\/div>\r\n\t&lt;input type=\&quot;hidden\&quot; name=\&quot;email-sablon-csatolmany\&quot; value=\&quot;\&quot; onchange=\&quot;_c1265c58d8f639545d556a813f7046ab_reverse_update()\&quot;>\r\n&lt;\/div>\r\n&lt;\/div>\r\n&lt;script data-eval>\r\nfunction _c1265c58d8f639545d556a813f7046ab_add() {\r\n\tif($('#email-sablon-csatolmany').val() > 0) {\r\n\t\t$('#email-sablon-csatolmany-labels').append('&lt;div class=\&quot;labels-label\&quot; data-id=\&quot;'+$('#email-sablon-csatolmany').val()+'\&quot;>&lt;span>'+$('#email-sablon-csatolmany option:selected').text()+'&lt;\/span>&lt;div class=\&quot;sm-remove\&quot; onclick=\&quot;_c1265c58d8f639545d556a813f7046ab_remove(this)\&quot;>&lt;i class=\&quot;fa fa-times\&quot;>&lt;\/i>&lt;\/div>&lt;\/div>')\r\n\t\t$('#email-sablon-csatolmany option:selected').remove()\r\n\t\t_c1265c58d8f639545d556a813f7046ab_update()\r\n\t}\r\n}\r\nfunction _c1265c58d8f639545d556a813f7046ab_remove(elem) {\r\n\t$('#email-sablon-csatolmany').append('&lt;option value=\&quot;'+$(elem).parents('.labels-label').attr('data-id')+'\&quot;>'+$(elem).parents('.labels-label').find('span').text()+'&lt;\/option>')\r\n\t$(elem).parents('.labels-label').remove()\r\n\t_c1265c58d8f639545d556a813f7046ab_update()\r\n}\r\nfunction _c1265c58d8f639545d556a813f7046ab_update() {\r\n\tvar ids = []\r\n\t$('#email-sablon-csatolmany-labels .labels-label').each(function(){\r\n\t\tids.push($(this).attr('data-id'))\r\n\t})\r\n\t$('input[name=\&quot;email-sablon-csatolmany\&quot;]').val(ids.join(','))\r\n\t_c1265c58d8f639545d556a813f7046ab_resize()\r\n}\r\nfunction _c1265c58d8f639545d556a813f7046ab_reverse_update() {\r\n\t$('#email-sablon-csatolmany-labels .labels-label').each(function(){\r\n\t\t$(this).find('.sm-remove').click()\r\n\t})\r\n\tvar ids = $('input[name=\&quot;email-sablon-csatolmany\&quot;]').val().split(',')\r\n\tfor(var i = 0; i &lt; ids.length; i++) {\r\n\t\tif(ids[i] > 0) {\r\n\t\t\t$('#email-sablon-csatolmany').val(ids[i])\r\n\t\t\t_c1265c58d8f639545d556a813f7046ab_add()\r\n\t\t\tconsole.log(ids[i])\r\n\t\t}\r\n\t}\r\n}\r\nfunction _c1265c58d8f639545d556a813f7046ab_resize() {\r\n\t$('#email-sablon-csatolmany').css('height', $('#email-sablon-csatolmany-labels').outerHeight() + 'px')\r\n\t$('#email-sablon-csatolmany').parents('.form-ctrl').css('height', $('#email-sablon-csatolmany-labels').outerHeight() + 'px')\r\n}\r\n&lt;\/script>&lt;script data-eval>components['email-sablon-csatolmany']={\&quot;label\&quot;:\&quot;Csatolm\\u00e1nyok (PDF nyomtatv\\u00e1ny)\&quot;,\&quot;labelSize\&quot;:\&quot;no\&quot;,\&quot;value\&quot;:\&quot;\&quot;,\&quot;options\&quot;:[\&quot;$client\&quot;,\&quot;listPdfNyomtatvany\&quot;,\&quot;PdfNyomtatvanyok\&quot;,{\&quot;PdfNyomtatvanyTipusID\&quot;:2,\&quot;dbName\&quot;:\&quot;B9019003\&quot;},{\&quot;PdfNyomtatvanyID\&quot;:\&quot;Nev\&quot;}],\&quot;_component\&quot;:\&quot;tags\&quot;,\&quot;_hashname\&quot;:\&quot;_c1265c58d8f639545d556a813f7046ab\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-csatolmany\&quot;};componentsLoaded['email-sablon-csatolmany']={};Object.assign(componentsLoaded['email-sablon-csatolmany'], components['email-sablon-csatolmany']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;\/div>&quot;,&quot;&lt;div class=\&quot;row auto-height\&quot;>&quot;,&quot;&lt;div data-component=\&quot;button\&quot; data-name=\&quot;email-sablon-torles\&quot; data-hash=\&quot;63a72bd16dda2f0249e428db511d6b4c\&quot;>&lt;div id=\&quot;email-sablon-torles\&quot; class=\&quot;button bb-green disabled\&quot; onclick=\&quot;if(!$(this).hasClass('disabled')){_1500cdbca00bdcd45824101ca9249b11_onclick(this)}\&quot;>\r\n\t&lt;i class=\&quot;fa fa-trash\&quot;>&lt;\/i>\r\n\t&lt;span>Sablon t\u00f6rl\u00e9se&lt;\/span>\r\n&lt;\/div>\r\n&lt;script data-eval>\r\n\tfunction _1500cdbca00bdcd45824101ca9249b11_onclick(that){\r\ndeleteEmailSablon()\r\n}\r\n&lt;\/script>\r\n&lt;script data-eval>components['email-sablon-torles']={\&quot;icon\&quot;:\&quot;trash\&quot;,\&quot;text\&quot;:\&quot;Sablon t\\u00f6rl\\u00e9se\&quot;,\&quot;color\&quot;:\&quot;green\&quot;,\&quot;disabled\&quot;:true,\&quot;onclick\&quot;:\&quot;deleteEmailSablon()\&quot;,\&quot;_component\&quot;:\&quot;button\&quot;,\&quot;_hashname\&quot;:\&quot;_1500cdbca00bdcd45824101ca9249b11\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-torles\&quot;};componentsLoaded['email-sablon-torles']={};Object.assign(componentsLoaded['email-sablon-torles'], components['email-sablon-torles']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;div data-component=\&quot;button\&quot; data-name=\&quot;email-sablon-mentes-uj\&quot; data-hash=\&quot;5229bf4742d72d32dc91a4651f4a883b\&quot;>&lt;div id=\&quot;email-sablon-mentes-uj\&quot; class=\&quot;button bb-green disabled right mr-0\&quot; onclick=\&quot;if(!$(this).hasClass('disabled')){_ba39ef2d8384bd6ecfd6271470fe8aab_onclick(this)}\&quot;>\r\n\t&lt;i class=\&quot;fa fa-copy\&quot;>&lt;\/i>\r\n\t&lt;span>Ment\u00e9s \u00fajk\u00e9nt&lt;\/span>\r\n&lt;\/div>\r\n&lt;script data-eval>\r\n\tfunction _ba39ef2d8384bd6ecfd6271470fe8aab_onclick(that){\r\nsaveEmailSablon(-1)\r\n}\r\n&lt;\/script>\r\n&lt;script data-eval>components['email-sablon-mentes-uj']={\&quot;icon\&quot;:\&quot;copy\&quot;,\&quot;text\&quot;:\&quot;Ment\\u00e9s \\u00fajk\\u00e9nt\&quot;,\&quot;color\&quot;:\&quot;green\&quot;,\&quot;class\&quot;:\&quot;right mr-0\&quot;,\&quot;disabled\&quot;:true,\&quot;onclick\&quot;:\&quot;saveEmailSablon(-1)\&quot;,\&quot;_component\&quot;:\&quot;button\&quot;,\&quot;_hashname\&quot;:\&quot;_ba39ef2d8384bd6ecfd6271470fe8aab\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-mentes-uj\&quot;};componentsLoaded['email-sablon-mentes-uj']={};Object.assign(componentsLoaded['email-sablon-mentes-uj'], components['email-sablon-mentes-uj']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;div data-component=\&quot;button\&quot; data-name=\&quot;email-sablon-mentes\&quot; data-hash=\&quot;08f227bf998b78925eb68019c2abc822\&quot;>&lt;div id=\&quot;email-sablon-mentes\&quot; class=\&quot;button bb-green disabled right\&quot; onclick=\&quot;if(!$(this).hasClass('disabled')){_ce2c8f8061a4dffa8be43e31e9e63ccc_onclick(this)}\&quot;>\r\n\t&lt;i class=\&quot;fa fa-save\&quot;>&lt;\/i>\r\n\t&lt;span>Sablon ment\u00e9se&lt;\/span>\r\n&lt;\/div>\r\n&lt;script data-eval>\r\n\tfunction _ce2c8f8061a4dffa8be43e31e9e63ccc_onclick(that){\r\nsaveEmailSablon()\r\n}\r\n&lt;\/script>\r\n&lt;script data-eval>components['email-sablon-mentes']={\&quot;icon\&quot;:\&quot;save\&quot;,\&quot;text\&quot;:\&quot;Sablon ment\\u00e9se\&quot;,\&quot;color\&quot;:\&quot;green\&quot;,\&quot;class\&quot;:\&quot;right\&quot;,\&quot;disabled\&quot;:true,\&quot;onclick\&quot;:\&quot;saveEmailSablon()\&quot;,\&quot;_component\&quot;:\&quot;button\&quot;,\&quot;_hashname\&quot;:\&quot;_ce2c8f8061a4dffa8be43e31e9e63ccc\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;email-sablon-mentes\&quot;};componentsLoaded['email-sablon-mentes']={};Object.assign(componentsLoaded['email-sablon-mentes'], components['email-sablon-mentes']);&lt;\/script>&lt;\/div>&quot;,&quot;&lt;\/div>&quot;],&quot;_component&quot;:&quot;wrapper&quot;,&quot;_hashname&quot;:&quot;_4d6b6d40f6a812184ae78bc7c3e297d5&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;email-modal&quot;};componentsLoaded['email-modal']={};Object.assign(componentsLoaded['email-modal'], components['email-modal']);
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;email-sablonok&quot;)/div[@class=&quot;inner&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='email-sablonok']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='IP címek'])[2]/following::div[5]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='IP címek'])[1]/following::div[7]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div[2]/div/div</value>
   </webElementXpaths>
</WebElementEntity>

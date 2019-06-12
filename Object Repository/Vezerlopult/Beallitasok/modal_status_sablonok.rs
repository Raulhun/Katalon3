<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>modal_status_sablonok</name>
   <tag></tag>
   <elementGuidId>9077f887-83f5-4446-9c99-d1b5a066cf40</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='status-sablonok']/div</value>
      </entry>
      <entry>
         <key>BASIC</key>
         <value>id(&quot;status-sablonok&quot;)/div[@class=&quot;inner&quot;]</value>
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


	&lt;Új sablon>
	_
	____________fas
	__2
	111
	11111
	2144123
	22
	88888
	aaa
	Alapsablon
	asd
	asd
	asdasdasdasd
	hhh
	l,él
	Manager szűrés
	Taszt
	teszt 1
	teszt 2
	Új sablon 2
	Új sablon 8

components['status-sablon-select']={&quot;options&quot;:[&quot;$client&quot;,&quot;listStatusSablon&quot;,&quot;StatusSablonok&quot;,{&quot;Aktiv&quot;:-1,&quot;dbName&quot;:&quot;B9019003&quot;},&quot;StatusSablonID&quot;],&quot;name&quot;:&quot;Nev&quot;,&quot;metadata&quot;:{&quot;data-active&quot;:&quot;$Aktiv&quot;},&quot;onchange&quot;:[&quot;updateStatusTemplateInputs(this)&quot;,&quot;setComponentInput('status-steps', 'data', 'StatusSablonID', $(this).val())&quot;,&quot;refreshComponent('status-steps')&quot;],&quot;defaults&quot;:{&quot;-1&quot;:&quot;&lt;\u00daj sablon>&quot;},&quot;_component&quot;:&quot;select&quot;,&quot;_hashname&quot;:&quot;_55a73b90da7f7ef8f84308e183cd5373&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;status-sablon-select&quot;};componentsLoaded['status-sablon-select']={};Object.assign(componentsLoaded['status-sablon-select'], components['status-sablon-select']);



	Sablon neve
	
	

components['status-template-name']={&quot;name&quot;:&quot;status-template-name&quot;,&quot;input&quot;:{&quot;label&quot;:&quot;Sablon neve&quot;,&quot;required&quot;:true},&quot;_component&quot;:&quot;inputs.text&quot;,&quot;_hashname&quot;:&quot;_22315cadf0864b1191a1bfe1f369d23a&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;status-template-name&quot;};componentsLoaded['status-template-name']={};Object.assign(componentsLoaded['status-template-name'], components['status-template-name']);



	Aktív
	
		
		
	

components['status-template-active']={&quot;name&quot;:&quot;status-template-active&quot;,&quot;input&quot;:{&quot;label&quot;:&quot;Akt\u00edv&quot;,&quot;class&quot;:&quot;shadow&quot;},&quot;_component&quot;:&quot;inputs.checkbox&quot;,&quot;_hashname&quot;:&quot;_5c3386731db13063c07b147561363793&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;status-template-active&quot;};componentsLoaded['status-template-active']={};Object.assign(componentsLoaded['status-template-active'], components['status-template-active']);



	
	Sablon mentése


	function _a429a15f5e424934d325ca07954c054a_onclick(that){
saveStatusTemplate()
}

components['status-sablon-mentes']={&quot;text&quot;:&quot;Sablon ment\u00e9se&quot;,&quot;icon&quot;:&quot;save&quot;,&quot;color&quot;:&quot;green&quot;,&quot;onclick&quot;:&quot;saveStatusTemplate()&quot;,&quot;_component&quot;:&quot;button&quot;,&quot;_hashname&quot;:&quot;_a429a15f5e424934d325ca07954c054a&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;status-sablon-mentes&quot;};componentsLoaded['status-sablon-mentes']={};Object.assign(componentsLoaded['status-sablon-mentes'], components['status-sablon-mentes']);



	
		
			Név
			
				Név
			
		
		
			Ikon
			
				Ikon
			
		
		
			
			
				
			
		
	
	
		
			
		
		
			
		
		
			
		
	


$('#new-status-step').parent().scroll(function(){
	$('#new-status-step').find('tr:first-child th div').css('top',$('#new-status-step').parent().scrollTop()+'px');
})

function _5b5180b7dfa3067441f97f24d5d75407_scroll() {
	$('#new-status-step').find('tr:first-child th div').css('top',$('#new-status-step').parent().scrollTop()+'px');
}

components['new-status-step']={&quot;data&quot;:[{&quot;Nev&quot;:&quot;&lt;input type=\&quot;text\&quot; name=\&quot;new-Nev\&quot; required>&quot;,&quot;Icon&quot;:&quot;&lt;div class=\&quot;icon-box-container shadow\&quot; onclick=\&quot;iconSelector(this)\&quot;>&lt;input type=\&quot;hidden\&quot; name=\&quot;new-Icon\&quot; value=\&quot;tasks\&quot;>&lt;input type=\&quot;hidden\&quot; name=\&quot;new-Icon\&quot; value=\&quot;tasks\&quot; onchange=\&quot;$(this).prev('input').val($(this).val())\&quot;>&lt;i class=\&quot;fa fa-tasks icon-box-icon\&quot;\&quot;>&lt;\/i>&lt;\/div>&quot;,&quot;Button&quot;:&quot;&lt;i id=\&quot;save-new-status-step\&quot; class=\&quot;fa fa-save fa-button bb-green disabled\&quot; onclick=\&quot;if(!$(this).hasClass('disabled')){insertStatusStep()}\&quot; title=\&quot;ment\u00e9s\&quot;>&lt;\/i>&quot;}],&quot;datatype&quot;:{&quot;Nev&quot;:{&quot;header&quot;:&quot;N\u00e9v&quot;},&quot;Icon&quot;:{&quot;header&quot;:&quot;Ikon&quot;},&quot;Button&quot;:{&quot;header&quot;:&quot;&quot;}},&quot;_component&quot;:&quot;table&quot;,&quot;_hashname&quot;:&quot;_5b5180b7dfa3067441f97f24d5d75407&quot;,&quot;_attributes&quot;:{&quot;class&quot;:&quot;wrapped&quot;,&quot;style&quot;:&quot;margin-bottom:10px&quot;},&quot;_name&quot;:&quot;new-status-step&quot;};componentsLoaded['new-status-step']={};Object.assign(componentsLoaded['new-status-step'], components['new-status-step']);



	
		
			
		
		
			
		
		
			Név
			
			
				Név
				
			
		
		
			Ikon
			
				Ikon
			
		
		
			Aktív
			
				Aktív
			
		
		
			
		
	
	
		
	


$('#status-steps').parent().scroll(function(){
	$('#status-steps').find('tr:first-child th div').css('top',$('#status-steps').parent().scrollTop()+'px');
})

function _13312208cadd07b7bc09a5bc93aa0e7f_scroll() {
	$('#status-steps').find('tr:first-child th div').css('top',$('#status-steps').parent().scrollTop()+'px');
}

components['status-steps']={&quot;data&quot;:[&quot;$client&quot;,&quot;listStatusStepBySablonID&quot;,&quot;StatusStepek&quot;,{&quot;StatusSablonID&quot;:&quot;-1&quot;,&quot;Aktiv&quot;:-1,&quot;dbName&quot;:&quot;B9019003&quot;}],&quot;metadata&quot;:{&quot;data-sablon-id&quot;:&quot;$StatusSablonoID&quot;,&quot;data-id&quot;:&quot;$StatusStepID&quot;,&quot;data-name&quot;:&quot;$Nev&quot;,&quot;data-icon&quot;:&quot;$Icon&quot;,&quot;data-active&quot;:&quot;$Aktiv&quot;,&quot;data-order&quot;:&quot;$OrderNumber&quot;},&quot;datatype&quot;:{&quot;Nev&quot;:{&quot;header&quot;:&quot;N\u00e9v&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Icon&quot;:{&quot;header&quot;:&quot;Ikon&quot;,&quot;filter&quot;:&quot;icon&quot;,&quot;attributes&quot;:{&quot;data-type&quot;:&quot;icon&quot;}},&quot;Aktiv&quot;:{&quot;header&quot;:&quot;Akt\u00edv&quot;,&quot;filter&quot;:&quot;bool&quot;,&quot;attributes&quot;:{&quot;data-type&quot;:&quot;checkbox&quot;}}},&quot;controlsBefore&quot;:{&quot;chevron-up&quot;:&quot;moveUp(this)&quot;,&quot;chevron-down&quot;:&quot;moveDown(this)&quot;},&quot;controlsAfter&quot;:{&quot;edit bb-green&quot;:&quot;editTableRow(this, 'status-sablon-form', {\r\n\t\t\t\t\t\t\t'StatusStepID' : 'data-id',\r\n\t\t\t\t\t\t\t'OrderNumber'  : 'data-order',\r\n\t\t\t\t\t\t\t'Nev'          : 'data-name',\r\n\t\t\t\t\t\t\t'Icon'         : 'data-icon',\r\n\t\t\t\t\t\t\t'Aktiv'        : 'data-active',\r\n\t\t\t\t\t\t}, {\r\n\t\t\t\t\t\t\t'Nev'          : 'data-name',\r\n\t\t\t\t\t\t\t'Icon'         : 'data-icon',\r\n\t\t\t\t\t\t\t'Aktiv'        : 'data-active',\r\n\t\t\t\t\t\t})&quot;},&quot;attributes&quot;:{&quot;class&quot;:&quot;tbl fixhead&quot;,&quot;cellspacing&quot;:&quot;0&quot;,&quot;cellpadding&quot;:&quot;0&quot;,&quot;style&quot;:&quot;margin-bottom:0&quot;},&quot;_component&quot;:&quot;table&quot;,&quot;_hashname&quot;:&quot;_13312208cadd07b7bc09a5bc93aa0e7f&quot;,&quot;_attributes&quot;:{&quot;class&quot;:&quot;wrapped&quot;,&quot;style&quot;:&quot;overflow:auto;height:258px;&quot;},&quot;_name&quot;:&quot;status-steps&quot;};componentsLoaded['status-steps']={};Object.assign(componentsLoaded['status-steps'], components['status-steps']);

</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;status-sablonok&quot;)/div[@class=&quot;inner&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='status-sablonok']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Sablon mentése'])[2]/following::div[3]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Sablon törlése'])[2]/following::div[5]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div[4]/div/div</value>
   </webElementXpaths>
</WebElementEntity>

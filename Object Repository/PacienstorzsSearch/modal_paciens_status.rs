<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>modal_paciens_status</name>
   <tag></tag>
   <elementGuidId>aa491b24-e13b-46a5-92d6-6e727bedf13c</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='search-by-status']/div</value>
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
Kattintással válassza ki a státuszt, amelyre szűrni szeretne!

	__2
	111
	aaa
	Alapsablon
	hhh
	l,él
	Manager szűrés
	Taszt
	teszt 2

components['status-sablon-for-search']={&quot;options&quot;:{&quot;15&quot;:&quot;__2&quot;,&quot;16&quot;:&quot;111&quot;,&quot;8&quot;:&quot;aaa&quot;,&quot;1&quot;:&quot;Alapsablon&quot;,&quot;17&quot;:&quot;hhh&quot;,&quot;9&quot;:&quot;l,\u00e9l&quot;,&quot;2&quot;:&quot;Manager sz\u0171r\u00e9s&quot;,&quot;5&quot;:&quot;Taszt&quot;,&quot;4&quot;:&quot;teszt 2&quot;},&quot;onchange&quot;:[&quot;setComponentInput('status-steps-for-search', 'data', 'StatusSablonID', $(this).val())&quot;,&quot;refreshComponent('status-steps-for-search')&quot;],&quot;_component&quot;:&quot;select&quot;,&quot;_hashname&quot;:&quot;_ad561eb561da3369b1b1c4add6fc9ec0&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;status-sablon-for-search&quot;};componentsLoaded['status-sablon-for-search']={};Object.assign(componentsLoaded['status-sablon-for-search'], components['status-sablon-for-search']);


	
		
			
			
				
			
		
		
			Státusz
			
				Státusz
			
		
	
	
		
			
		
		
			1
		
	
	
		
			
		
		
			2
		
	


$('#status-steps-for-search').parent().scroll(function(){
	$('#status-steps-for-search').find('tr:first-child th div').css('top',$('#status-steps-for-search').parent().scrollTop()+'px');
})

function _aef5318f01465159cf109863f7ecdc16_scroll() {
	$('#status-steps-for-search').find('tr:first-child th div').css('top',$('#status-steps-for-search').parent().scrollTop()+'px');
}

components['status-steps-for-search']={&quot;data&quot;:[&quot;$client&quot;,&quot;listStatusStepBySablonID&quot;,&quot;StatusStepek&quot;,{&quot;StatusSablonID&quot;:15,&quot;Aktiv&quot;:1,&quot;dbName&quot;:&quot;B9019003&quot;}],&quot;metadata&quot;:{&quot;data-sablon&quot;:&quot;$StatusSablonID&quot;,&quot;data-step&quot;:&quot;$StatusStepID&quot;,&quot;class&quot;:&quot;selectable sablon&quot;,&quot;onclick&quot;:&quot;searchPatientBySablon(this)&quot;},&quot;datatype&quot;:{&quot;Icon&quot;:{&quot;filter&quot;:&quot;icon&quot;,&quot;header-attributes&quot;:{&quot;style&quot;:&quot;width:30px&quot;}},&quot;Nev&quot;:{&quot;header&quot;:&quot;St\u00e1tusz&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}}},&quot;_component&quot;:&quot;table&quot;,&quot;_hashname&quot;:&quot;_aef5318f01465159cf109863f7ecdc16&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;status-steps-for-search&quot;};componentsLoaded['status-steps-for-search']={};Object.assign(componentsLoaded['status-steps-for-search'], components['status-steps-for-search']);

</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;content&quot;)/div[@id=&quot;ajaxcontent&quot;]/div[11]/div[@id=&quot;search-by-status&quot;]/div[@class=&quot;inner&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='search-by-status']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Felvétel dátuma'])[5]/following::div[5]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Felvétel dátuma'])[4]/following::div[6]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[11]/div/div</value>
   </webElementXpaths>
</WebElementEntity>

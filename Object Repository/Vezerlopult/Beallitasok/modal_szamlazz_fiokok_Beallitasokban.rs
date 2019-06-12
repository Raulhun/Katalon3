<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>modal_szamlazz_fiokok_Beallitasokban</name>
   <tag></tag>
   <elementGuidId>b164cf98-ac0a-4644-a963-907c5119b059</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='szamlazz-fiokok-modal']/div</value>
      </entry>
      <entry>
         <key>BASIC</key>
         <value>id(&quot;szamlazz-fiokok-modal&quot;)/div[@class=&quot;inner&quot;]</value>
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

	- - Új Számlázz.hu fiók - -
	,hj,
	Abc
	b
	másodlagos fiók
	q
	sssss
	v

components['szamlazz-fiok-select']={&quot;defaults&quot;:{&quot;-1&quot;:&quot;- - \u00daj Sz\u00e1ml\u00e1zz.hu fi\u00f3k - -&quot;},&quot;options&quot;:[&quot;$client&quot;,&quot;listSzamlazzAccount&quot;,&quot;SzamlazzAccount&quot;,{&quot;dbName&quot;:&quot;B9019003&quot;},&quot;SzamlazzAccountID&quot;],&quot;name&quot;:&quot;AccountNev&quot;,&quot;metadata&quot;:{&quot;data-id&quot;:&quot;$SzamlazzAccountID&quot;,&quot;data-account&quot;:&quot;$AccountNev&quot;,&quot;data-user&quot;:&quot;$UserNev&quot;,&quot;data-pass&quot;:&quot;$Password&quot;,&quot;data-bank&quot;:&quot;$Bank&quot;,&quot;data-number&quot;:&quot;$Szamlaszam&quot;},&quot;onchange&quot;:&quot;changeSzamlazzFiok(this)&quot;,&quot;onRefresh&quot;:&quot;$(this).change()&quot;,&quot;_component&quot;:&quot;select&quot;,&quot;_hashname&quot;:&quot;_ee1b3a802fbf969bafba00f963470844&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;szamlazz-fiok-select&quot;,&quot;_refresh&quot;:&quot;__ee1b3a802fbf969bafba00f963470844&quot;};componentsLoaded['szamlazz-fiok-select']={};Object.assign(componentsLoaded['szamlazz-fiok-select'], components['szamlazz-fiok-select']);function __ee1b3a802fbf969bafba00f963470844(){$(this).change()}

	
		
				
	Fiók neve
	
	

		
	Számlázz.hu felhasználónév
	
	

		
			Számlázz.hu jelszó
			
		
		
			Bank neve
			
		
		
	Számlaszám
	
	

		
		
		
	
	Mentés


	function _9262c1f045b54df375b3be9652747697_onclick(that){
saveSzamlazzFiok()
}

components['szamlazz-fiok-mentes']={&quot;text&quot;:&quot;Ment\u00e9s&quot;,&quot;icon&quot;:&quot;save&quot;,&quot;color&quot;:&quot;green&quot;,&quot;onclick&quot;:&quot;saveSzamlazzFiok()&quot;,&quot;_component&quot;:&quot;button&quot;,&quot;_hashname&quot;:&quot;_9262c1f045b54df375b3be9652747697&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;szamlazz-fiok-mentes&quot;};componentsLoaded['szamlazz-fiok-mentes']={};Object.assign(componentsLoaded['szamlazz-fiok-mentes'], components['szamlazz-fiok-mentes']);
		

polyfill()components['szamlazz-fiok']={&quot;attributes&quot;:{&quot;method&quot;:&quot;post&quot;,&quot;action&quot;:&quot;updateszamlazzfiok&quot;,&quot;style&quot;:&quot;margin-top:10px&quot;},&quot;inputs&quot;:{&quot;0&quot;:&quot;&lt;div class=\&quot;row auto-height\&quot;>&quot;,&quot;SzamlazzAccountID&quot;:{&quot;type&quot;:&quot;hidden&quot;},&quot;AccountNev&quot;:{&quot;type&quot;:&quot;text&quot;,&quot;label&quot;:&quot;Fi\u00f3k neve&quot;},&quot;UserNev&quot;:{&quot;type&quot;:&quot;text&quot;,&quot;label&quot;:&quot;Sz\u00e1ml\u00e1zz.hu felhaszn\u00e1l\u00f3n\u00e9v&quot;},&quot;Password&quot;:{&quot;type&quot;:&quot;password&quot;,&quot;label&quot;:&quot;Sz\u00e1ml\u00e1zz.hu jelsz\u00f3&quot;},&quot;Bank&quot;:{&quot;type&quot;:&quot;bank&quot;,&quot;label&quot;:&quot;Bank neve&quot;},&quot;Szamlaszam&quot;:{&quot;type&quot;:&quot;text&quot;,&quot;label&quot;:&quot;Sz\u00e1mlasz\u00e1m&quot;},&quot;1&quot;:&quot;&lt;\/div>&quot;,&quot;2&quot;:&quot;&lt;div class=\&quot;row auto-height text-center\&quot;>&quot;,&quot;3&quot;:&quot;&lt;div data-component=\&quot;button\&quot; data-name=\&quot;szamlazz-fiok-mentes\&quot; data-hash=\&quot;1d109a4977580a5697d6f8950c40e1fb\&quot;>&lt;div id=\&quot;szamlazz-fiok-mentes\&quot; class=\&quot;button bb-green\&quot; onclick=\&quot;if(!$(this).hasClass('disabled')){_9262c1f045b54df375b3be9652747697_onclick(this)}\&quot;>\r\n\t&lt;i class=\&quot;fa fa-save\&quot;>&lt;\/i>\r\n\t&lt;span>Ment\u00e9s&lt;\/span>\r\n&lt;\/div>\r\n&lt;script data-eval>\r\n\tfunction _9262c1f045b54df375b3be9652747697_onclick(that){\r\nsaveSzamlazzFiok()\r\n}\r\n&lt;\/script>\r\n&lt;script data-eval>components['szamlazz-fiok-mentes']={\&quot;text\&quot;:\&quot;Ment\\u00e9s\&quot;,\&quot;icon\&quot;:\&quot;save\&quot;,\&quot;color\&quot;:\&quot;green\&quot;,\&quot;onclick\&quot;:\&quot;saveSzamlazzFiok()\&quot;,\&quot;_component\&quot;:\&quot;button\&quot;,\&quot;_hashname\&quot;:\&quot;_9262c1f045b54df375b3be9652747697\&quot;,\&quot;_attributes\&quot;:[],\&quot;_name\&quot;:\&quot;szamlazz-fiok-mentes\&quot;};componentsLoaded['szamlazz-fiok-mentes']={};Object.assign(componentsLoaded['szamlazz-fiok-mentes'], components['szamlazz-fiok-mentes']);&lt;\/script>&lt;\/div>&quot;,&quot;4&quot;:&quot;&lt;\/div>&quot;},&quot;_component&quot;:&quot;form&quot;,&quot;_hashname&quot;:&quot;_adbef77c05c7e7413fceefaafc457d72&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;szamlazz-fiok&quot;};componentsLoaded['szamlazz-fiok']={};Object.assign(componentsLoaded['szamlazz-fiok'], components['szamlazz-fiok']);
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;szamlazz-fiokok-modal&quot;)/div[@class=&quot;inner&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='szamlazz-fiokok-modal']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Aktív'])[4]/following::div[5]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Aktív'])[3]/following::div[6]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[6]/div/div</value>
   </webElementXpaths>
</WebElementEntity>

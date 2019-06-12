<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>modal_egeszsegpenztarak_Beallitasokban</name>
   <tag></tag>
   <elementGuidId>6dcf355a-d1b1-4efb-ab19-d3dc6cee3e77</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='egeszsegpenztarak-modal']/div</value>
      </entry>
      <entry>
         <key>BASIC</key>
         <value>id(&quot;egeszsegpenztarak-modal&quot;)/div[@class=&quot;inner widerr&quot;]</value>
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
      <value>inner widerr</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

	Összes
	Aktív
	Inaktív

components['egeszsegpenztar-aktiv-select']={&quot;options&quot;:{&quot;-1&quot;:&quot;\u00d6sszes&quot;,&quot;1&quot;:&quot;Akt\u00edv&quot;,&quot;0&quot;:&quot;Inakt\u00edv&quot;},&quot;onchange&quot;:[&quot;setComponentData('egeszsegpenztarak', 'read.params.Aktiv', $(this).val())&quot;,&quot;refreshComponent('egeszsegpenztarak')&quot;],&quot;selected&quot;:&quot;1&quot;,&quot;_component&quot;:&quot;select&quot;,&quot;_hashname&quot;:&quot;_05908c27541ba3220fe0080c1d31e2b9&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;egeszsegpenztar-aktiv-select&quot;};componentsLoaded['egeszsegpenztar-aktiv-select']={};Object.assign(componentsLoaded['egeszsegpenztar-aktiv-select'], components['egeszsegpenztar-aktiv-select']);

	
														
polyfill()components['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-form']={&quot;attributes&quot;:{&quot;method&quot;:&quot;post&quot;,&quot;action&quot;:&quot;save-epenztar&quot;,&quot;enctype&quot;:&quot;application\/x-www-form-urlencoded&quot;,&quot;style&quot;:&quot;display:none&quot;},&quot;inputs&quot;:{&quot;PenztarID&quot;:{&quot;type&quot;:&quot;hidden&quot;},&quot;Nev&quot;:{&quot;type&quot;:&quot;hidden&quot;},&quot;Iranyitoszam&quot;:{&quot;type&quot;:&quot;hidden&quot;},&quot;Telepules&quot;:{&quot;type&quot;:&quot;hidden&quot;},&quot;Cim&quot;:{&quot;type&quot;:&quot;hidden&quot;},&quot;Adoszam&quot;:{&quot;type&quot;:&quot;hidden&quot;},&quot;Aktiv&quot;:{&quot;type&quot;:&quot;hidden&quot;}},&quot;_component&quot;:&quot;form&quot;,&quot;_hashname&quot;:&quot;_d2d14cad5d19c2010227495d06e7827d&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f-form&quot;};componentsLoaded['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-form']={};Object.assign(componentsLoaded['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-form'], components['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-form']);
    
window._9ba8a620741907d52e1377bd39c946bf_undo = null
function _342edbf3b5a0d7a1bf4b8cdd1dfca70f_edit(elem) {
    _9ba8a620741907d52e1377bd39c946bf_editTableRow(elem, '_342edbf3b5a0d7a1bf4b8cdd1dfca70f-form', {&quot;PenztarID&quot;:&quot;data-id&quot;,&quot;Nev&quot;:&quot;data-name&quot;,&quot;Iranyitoszam&quot;:&quot;data-iranyitoszam&quot;,&quot;Telepules&quot;:&quot;data-telepules&quot;,&quot;Cim&quot;:&quot;data-cim&quot;,&quot;Adoszam&quot;:&quot;data-adoszam&quot;,&quot;Aktiv&quot;:&quot;data-aktiv&quot;}, $(elem).hasClass('new') ? {&quot;Nev&quot;:&quot;data-name&quot;,&quot;Iranyitoszam&quot;:&quot;data-iranyitoszam&quot;,&quot;Telepules&quot;:&quot;data-telepules&quot;,&quot;Cim&quot;:&quot;data-cim&quot;,&quot;Adoszam&quot;:&quot;data-adoszam&quot;} : {&quot;Nev&quot;:&quot;data-name&quot;,&quot;Iranyitoszam&quot;:&quot;data-iranyitoszam&quot;,&quot;Telepules&quot;:&quot;data-telepules&quot;,&quot;Cim&quot;:&quot;data-cim&quot;,&quot;Adoszam&quot;:&quot;data-adoszam&quot;,&quot;Aktiv&quot;:&quot;data-aktiv&quot;})
}
    function _9ba8a620741907d52e1377bd39c946bf_editTableRow(elem, targetForm, targetInputs, editableInputs, scriptBefore) {
	if($(elem).hasClass('disabled')) {
		return;
	}
	scriptBefore = scriptBefore === undefined ? '' : scriptBefore+';'
	var $row = $(elem).parents('tr')
	_9ba8a620741907d52e1377bd39c946bf_undo = $row.clone()
	$(elem).parents('table').find('.fa-button').addClass('disabled')
	if(!$(elem).hasClass('new')) {
		$(elem).parents('td').attr('style', 'width:96px!important').html('&lt;i class=&quot;fa fa-undo fa-button bb-green&quot; onclick=&quot;_9ba8a620741907d52e1377bd39c946bf_undoTableRow(this)&quot; title=&quot;Mégsem&quot;>&lt;/i>&lt;i class=&quot;fa fa-save bb-green fa-button&quot; onclick=&quot;'+scriptBefore+'_9ba8a620741907d52e1377bd39c946bf_saveTableRow(this, \''+targetForm+'\', \''+JSON.stringify(targetInputs).replace(/&quot;/g, '&amp;quot;')+'\')&quot; title=&quot;Mentés&quot;>&lt;/i>')
	}
	else {
		$(elem).parents('td').html('&lt;i class=&quot;fa fa-save bb-green fa-button new&quot; onclick=&quot;'+scriptBefore+'_9ba8a620741907d52e1377bd39c946bf_saveTableRow(this, \''+targetForm+'\', \''+JSON.stringify(targetInputs).replace(/&quot;/g, '&amp;quot;')+'\')&quot; title=&quot;Mentés&quot;>&lt;/i>')
	}
	var $form = $$(targetForm)
	$.each(targetInputs, function(k, v){
		if($form.find('input[name='+k+']').length) {
			if($row.find('td[data-key='+k+']').attr('data-type') == 'checkbox') {
				if($row.attr(v) == 'true') {
					$row.attr(v, 1)
				}
				else if($row.attr(v) == 'false') {
					$row.attr(v, 0)
				}
			}
			if($row.find('td[data-key='+k+']').attr('data-type') != 'file') {
				$form.find('input[name='+k+']').val($row.attr(v))
			}
		}
	})
	$.each(editableInputs, function(k, v){
		var val = typeof $row.attr(v) === 'undefined' ? '' : $row.attr(v)
		if($row.find('td[data-key='+k+']').length) {
            var req = typeof $row.find('td[data-key='+k+']').attr('data-required') === 'undefined' ? '' : ' required'
			var $td = $row.find('td[data-key='+k+']')
			switch($td.attr('data-type')) {

				case 'color':
					$td.html('&lt;div class=&quot;color-box-container&quot;>&lt;input id=&quot;editcolor&quot; type=&quot;color&quot; name=&quot;'+k+'&quot; value=&quot;#'+val+'&quot; onchange=&quot;$$(\''+targetForm+'\').find(\'input[name='+k+']\').val($(this).val())&quot;'+req+'>&lt;i class=&quot;fa fa-paint-brush color-box-icon&quot;>&lt;/i>&lt;/div>')
					polyfill()
					$('#editcolor').spectrum('set', '#'+val)
					break

				case 'number':
					$td.html('&lt;input type=&quot;number&quot; class=&quot;shadow&quot; style=&quot;width:80%!important;margin-left:10%;padding-left:5px&quot; name=&quot;'+k+'&quot; value=&quot;'+val+'&quot; min=&quot;0&quot; max=&quot;999999999&quot; step=&quot;1&quot; onchange=&quot;if($(this).val() == \'\' || isNaN($(this).val())){$(this).val($(this).attr(\'min\'))};$$(\''+targetForm+'\').find(\'input[name='+k+']\').val($(this).val())&quot;'+req+'>')
					break

				case 'date':
					$td.html('&lt;input type=&quot;date&quot; class=&quot;shadow&quot; name=&quot;'+k+'&quot; value=&quot;'+val.substring(0, 10)+'&quot;onchange=&quot;$$(\''+targetForm+'\').find(\'input[name='+k+']\').val($(this).val())&quot;'+req+'>')
					polyfill()
					break;

				case 'checkbox':
					$td.html('&lt;label class=&quot;checkbox check-box&quot;>&lt;input type=&quot;checkbox&quot; name=&quot;'+k+'&quot; onchange=&quot;$$(\''+targetForm+'\').find(\'input[name='+k+']\').val($(this).is(\':checked\') ? 1 : 0)&quot;'+(val == 'true' || val == 1 ? ' checked' : '')+req+'>&lt;span class=&quot;shadow&quot;>&lt;/span>&lt;/label>')
					break

				case 'select':
					var options = JSON.parse($td.attr('data-options').replace('&amp;quot;', '&quot;'))
					console.log(options);
					if(typeof options !== 'undefined') {
						__selected = val;
						options = $.map(options, function(val, key){
							return '&lt;option value=&quot;' + key + '&quot;'+(__selected == val ? ' selected' : '')+'>'+ val + '&lt;/option>';
						}).join('')
						$row.find('td[data-key='+k+']').html('&lt;select class=&quot;shadow&quot; name=&quot;'+k+'&quot; onchange=&quot;$$(\''+targetForm+'\').find(\'input[name='+k+']\').val($(this).val())&quot;>'+options+'&lt;/select>')
					}
					break;

				case 'file':
					if($(elem).hasClass('new')) {
						// méret ell.
						var acc = '';
						if(typeof $td.attr('data-accept') !== 'undefined') {
							acc = ' accept=&quot;'+$td.attr('data-accept')+'&quot;';
						}
						$td.html('&lt;input name=&quot;'+k+'&quot; type=&quot;file&quot; onchange=&quot;_9ba8a620741907d52e1377bd39c946bf_fileInput(this, \''+targetForm+'\')&quot;'+req+acc+'>')
					}
					break

				default:
					$td.html('&lt;input class=&quot;shadow&quot; type=&quot;text&quot; name=&quot;'+k+'&quot; value=&quot;'+val+'&quot; oninput=&quot;$$(\''+targetForm+'\').find(\'input[name='+k+']\').val($(this).val())&quot;'+req+'>')
					break
			}
		}
	})
}

function _9ba8a620741907d52e1377bd39c946bf_fileInput(elem, targetForm) {
	var $this = $(elem), $clone = $this.clone()
	$('#'+targetForm).find('input[name=&quot;'+$(elem).attr('name')+'&quot;]').remove()
	$this.after($clone).appendTo($('#'+targetForm))
}

function _9ba8a620741907d52e1377bd39c946bf_readFile(evt) {
	var $elem = $(evt.target)
	var files = evt.target.files;
	var file = files[0];
	var reader = new FileReader();
	$elem.parents('div[data-component=&quot;table&quot;]').addClass('loading')
	reader.onload = function(event) {
		$$($elem.attr('data-target')).find('input[name='+$elem.attr('name')+']').val(event.target.result)
		$elem.parents('div[data-component=&quot;table&quot;]').removeClass('loading')
	}
	reader.readAsBinaryString(file)
}

function _9ba8a620741907d52e1377bd39c946bf_undoTableRow(elem) {
	var $table = $(elem).parents('table')
	var $row = $(elem).parents('tr')
	$row.replaceWith(_9ba8a620741907d52e1377bd39c946bf_undo)
	$table.find('.fa-button').removeClass('disabled')
	_9ba8a620741907d52e1377bd39c946bf_undo = null
}
function _9ba8a620741907d52e1377bd39c946bf_saveTableRow(elem, targetForm, targetInputs) {
	if($(elem).parents('tr').find('input[name=Nev]').length &amp;&amp; $(elem).parents('tr').find('input[name=Nev]').val().trim() == '') {
		alert('A mentéshez előbb írjon be egy nevet!')
		return
	}
	var $row = $(elem).parents('tr')
	var $form = $$(targetForm)
	if($(elem).hasClass('new')) {
		targetInputs = JSON.parse(targetInputs)
		$.each(targetInputs, function(k, v){
			if($row.find('input[name='+k+']').attr('type') !== 'file') {
				if($form.find('input[name='+k+'],select[name='+k+']').length &amp;&amp; $row.find('input[name='+k+'],select[name='+k+']').length) {
					$form.find('input[name='+k+'],select[name='+k+']').val($row.find('input[name='+k+'],select[name='+k+']').val())
				}
				else if($form.find('input[name='+k+']').length) {
					$form.find('input[name='+k+']').val($row.attr(v))
				}
			}
		})
	}
	$(elem).parents('tr').find('input,select').attr('disabled', 'disabled')
	$(elem).parents('td').find('.fa-button').addClass('disabled')
	setTimeout(function() {
		submitForm('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-form', function(){
			showFlash('1Sikeres mentés!')
			refreshComponent($(elem).parents('table').attr('id'))
		}, function(){
			alert('Ismeretlen hiba történt a mentés során, kérjük próbálja újra!')
			refreshComponent($(elem).parents('table').attr('id'))
		})
	}, 10)
}
    
components['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-crud-script']={&quot;table&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table&quot;,&quot;function&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f_edit&quot;,&quot;form&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f-form&quot;,&quot;action&quot;:&quot;save-epenztar&quot;,&quot;delete&quot;:null,&quot;upload&quot;:null,&quot;data&quot;:{&quot;PenztarID&quot;:&quot;data-id&quot;,&quot;Nev&quot;:&quot;data-name&quot;,&quot;Iranyitoszam&quot;:&quot;data-iranyitoszam&quot;,&quot;Telepules&quot;:&quot;data-telepules&quot;,&quot;Cim&quot;:&quot;data-cim&quot;,&quot;Adoszam&quot;:&quot;data-adoszam&quot;,&quot;Aktiv&quot;:&quot;data-aktiv&quot;},&quot;editable&quot;:{&quot;Nev&quot;:&quot;data-name&quot;,&quot;Iranyitoszam&quot;:&quot;data-iranyitoszam&quot;,&quot;Telepules&quot;:&quot;data-telepules&quot;,&quot;Cim&quot;:&quot;data-cim&quot;,&quot;Adoszam&quot;:&quot;data-adoszam&quot;,&quot;Aktiv&quot;:&quot;data-aktiv&quot;},&quot;editableNew&quot;:{&quot;Nev&quot;:&quot;data-name&quot;,&quot;Iranyitoszam&quot;:&quot;data-iranyitoszam&quot;,&quot;Telepules&quot;:&quot;data-telepules&quot;,&quot;Cim&quot;:&quot;data-cim&quot;,&quot;Adoszam&quot;:&quot;data-adoszam&quot;},&quot;_component&quot;:&quot;crud-script&quot;,&quot;_hashname&quot;:&quot;_9ba8a620741907d52e1377bd39c946bf&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f-crud-script&quot;};componentsLoaded['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-crud-script']={};Object.assign(componentsLoaded['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-crud-script'], components['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-crud-script']);
                        
	
		
			Név
			
			
				Név
				
			
		
		
			Irányítószám
			
			
				Irányítószám
				
			
		
		
			Település
			
			
				Település
				
			
		
		
			Cím
			
			
				Cím
				
			
		
		
			Adószám
			
			
				Adószám
				
			
		
		
			
		
	
	
		
		
		
		
		
		
	


$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new').parent().scroll(function(){
	$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new').find('tr:first-child th div').css('top',$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new').parent().scrollTop()+'px');
})

function _f47ba56efde34073151eede1a71c3bd9_scroll() {
	$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new').find('tr:first-child th div').css('top',$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new').parent().scrollTop()+'px');
}

components['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new']={&quot;data&quot;:[{&quot;PenztarID&quot;:&quot;-1&quot;,&quot;Nev&quot;:&quot;&quot;,&quot;Iranyitoszam&quot;:&quot;&quot;,&quot;Telepules&quot;:&quot;&quot;,&quot;Cim&quot;:&quot;&quot;,&quot;Adoszam&quot;:&quot;&quot;}],&quot;metadata&quot;:{&quot;data-id&quot;:&quot;$PenztarID&quot;,&quot;data-name&quot;:&quot;$Nev&quot;,&quot;data-iranyitoszam&quot;:&quot;$Iranyitoszam&quot;,&quot;data-telepules&quot;:&quot;$Telepules&quot;,&quot;data-cim&quot;:&quot;$Cim&quot;,&quot;data-adoszam&quot;:&quot;$Adoszam&quot;,&quot;data-aktiv&quot;:&quot;$Aktiv&quot;},&quot;datatype&quot;:{&quot;Nev&quot;:{&quot;header&quot;:&quot;N\u00e9v&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;,&quot;data-required&quot;:true},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Iranyitoszam&quot;:{&quot;header&quot;:&quot;Ir\u00e1ny\u00edt\u00f3sz\u00e1m&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Telepules&quot;:{&quot;header&quot;:&quot;Telep\u00fcl\u00e9s&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Cim&quot;:{&quot;header&quot;:&quot;C\u00edm&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Adoszam&quot;:{&quot;header&quot;:&quot;Ad\u00f3sz\u00e1m&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}}},&quot;controlsAfter&quot;:{&quot;edit bb-green new&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f_edit(this)&quot;},&quot;controlTitles&quot;:{&quot;edit bb-green new&quot;:&quot;Ment\u00e9s&quot;},&quot;settings&quot;:{&quot;sortable&quot;:true,&quot;editable&quot;:true},&quot;onRefresh&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f_ref();refreshComponent('_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table')&quot;,&quot;_component&quot;:&quot;table&quot;,&quot;_hashname&quot;:&quot;_f47ba56efde34073151eede1a71c3bd9&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new&quot;,&quot;_refresh&quot;:&quot;__f47ba56efde34073151eede1a71c3bd9&quot;};componentsLoaded['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new']={};Object.assign(componentsLoaded['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new'], components['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new']);function __f47ba56efde34073151eede1a71c3bd9(){_342edbf3b5a0d7a1bf4b8cdd1dfca70f_ref();refreshComponent('_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table')}
    
    _342edbf3b5a0d7a1bf4b8cdd1dfca70f_ref()
    function _342edbf3b5a0d7a1bf4b8cdd1dfca70f_ref(){
        $('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table-new td:not([data-key]) .fa-button').click()
    }
    
                        
	
		
			Név
			
			
				Név
				
			
		
		
			Adószám
			
			
				Adószám
				
			
		
		
			Irányítószám
			
			
				Irányítószám
				
			
		
		
			Település
			
			
				Település
				
			
		
		
			Cím
			
			
				Cím
				
			
		
		
			Aktív
			
				Aktív
			
		
		
			
		
	
	
		
			_teszt_Pénztár lll
		
		
			99999999-2-08
		
		
			9021 
		
		
			Kiskunkiabánattudjamicsoda hhhh ffff
		
		
			2222222222   ééé meg még sok minden nemtommi
		
		
			
		
		
			
		
	
	
		
			_zzzzzzzzzzzz
		
		
			
			000000000000
		
		
			     
		
		
			
			000000000000
		
		
			
			000000000000
		
		
			
		
		
			
		
	
	
		
			AAAA
		
		
			AAA
		
		
			AAA  
		
		
			AAA
		
		
			AAA
		
		
			
		
		
			
		
	
	
		
			AAAAAAAAA
		
		
			
			000000000000
		
		
			     
		
		
			
			000000000000
		
		
			
			000000000000
		
		
			
		
		
			
		
	
	
		
			Allianz Hungária Önkéntes Kölcsönös Egészség- és Önsegélyező Pénztár
		
		
			18116870142
		
		
			1087 
		
		
			Budapest
		
		
			Könyves Kálmán körút 48-52.
		
		
			
		
		
			
		
	
	
		
			DANUBIUS Egészség-és Önsegélyező Pénztár
		
		
			18111026141
		
		
			1051 
		
		
			Budapest
		
		
			Szent István tér 11.
		
		
			
		
		
			
		
	
	
		
			ddd
		
		
			
			000000000000
		
		
			     
		
		
			
			000000000000
		
		
			
			000000000000
		
		
			
		
		
			
		
	
	
		
			Egészségért Országos Önkéntes Egészségpénztár
		
		
			18118573141
		
		
			1037 
		
		
			Budapest
		
		
			Kunigunda útja 60.
		
		
			
		
		
			
		
	
	
		
			Generali Egészség- és Önsegélyező Pénztár
		
		
			18177796242
		
		
			1066 
		
		
			Budapest
		
		
			Teréz körút 42-44.
		
		
			
		
		
			
		
	
	
		
			Herendi Porcelánmanufaktúra Rt. Egészségpénztára
		
		
			18931123119
		
		
			8440 
		
		
			Herend
		
		
			Kossuth L. utca 140.
		
		
			
		
		
			
		
	
	
		
			Kardirex Egészségpénztár
		
		
			18981173-1-08
		
		
			9024 
		
		
			Győr
		
		
			Táncsics Mihály út 43
		
		
			
		
		
			
		
	
	
		
			KARDIREX Önkéntes Kölcsönös Kiegészítő Egészség és Önsegélyező Pénztár
		
		
			18981173108
		
		
			9024 
		
		
			Győr
		
		
			Táncsics Mihály utca 43.
		
		
			
		
		
			
		
	
	
		
			kkkkkk
		
		
			
			000000000000
		
		
			kkkkk
		
		
			
			000000000000
		
		
			
			000000000000
		
		
			
		
		
			
		
	
	
		
			Medicina Egészségpénztár
		
		
			18097560241
		
		
			1037 
		
		
			Budapest
		
		
			Montevideo utca 5.
		
		
			
		
		
			
		
	
	
		
			MKB - Pannónia Egészség- és Önsegélyező Pénztár
		
		
			18232761141
		
		
			1056 
		
		
			Budapest
		
		
			Váci utca 38.
		
		
			
		
		
			
		
	
	
		
			MKB Egészség- és Önsegélyező Pénztár
		
		
			18232761141
		
		
			1056 
		
		
			Budapest
		
		
			Váci utca 38.
		
		
			
		
		
			
		
	
	
		
			OTP Országos Egészség- és Önsegélyező Pénztár
		
		
			18105564141
		
		
			1133 
		
		
			Budapest
		
		
			Váci út 76.
		
		
			
		
		
			
		
	
	
		
			Patika Önkéntes Kölcsönös Egészség- és Önsegélyező Pénztár
		
		
			18238949141
		
		
			1022 
		
		
			Budapest
		
		
			Bimbó út 18.
		
		
			
		
		
			
		
	
	
		
			PRÉMIUM Egészségpénztár
		
		
			18177734241
		
		
			1138 
		
		
			Budapest
		
		
			Dunavirág utca 2-6.
		
		
			
		
		
			
		
	
	
		
			qqq
		
		
			2222202
		
		
			     
		
		
			
			000000000000
		
		
			
			000000000000
		
		
			
		
		
			
		
	
	
		
			RTeszt
		
		
			2222222
		
		
			bsfb 
		
		
			bs
		
		
			bsb
		
		
			
		
		
			
		
	
	
		
			SSSSSSSS
		
		
			SSSSSSSSSSS
		
		
			SSSSS
		
		
			SSSSSSSSSSSSSS
		
		
			SSSSSSSSSSS
		
		
			
		
		
			
		
	
	
		
			TEMPO Egészség- és Önsegélyező Pénztár
		
		
			18229343241
		
		
			1025 
		
		
			Budapest
		
		
			Nagybányai utca 92.
		
		
			
		
		
			
		
	
	
		
			TEST-VÉR Magánbiztosító Egészségpénztár
		
		
			18232273143
		
		
			1119 
		
		
			Budapest
		
		
			Hengermalom út 6. A. épület FSZ. lépcsőház 3. emelt
		
		
			
		
		
			
		
	
	
		
			Tradíció Önkéntes Kölcsönös Kiegészítő Egészség- és Önsegélyező Pénztár
		
		
			18189065141
		
		
			1022 
		
		
			Budapest
		
		
			Bimbó út 18.
		
		
			
		
		
			
		
	
	
		
			Új Pillér Önkéntes Kölcsönös Kiegészítő Egészség- és Önsegélyező Pénztár
		
		
			18189072141
		
		
			1022 
		
		
			Budapest
		
		
			Bimbó út 18.
		
		
			
		
		
			
		
	
	
		
			uuuuuu
		
		
			
			000000000000
		
		
			tttt 
		
		
			
			000000000000
		
		
			
			000000000000
		
		
			
		
		
			
		
	
	
		
			Vasutas Önkéntes Kölcsönös Kiegészítő Egészség- és Önsegélyező Pénztár
		
		
			18157169142
		
		
			1144 
		
		
			Budapest
		
		
			Kőszeg utca 26.
		
		
			
		
		
			
		
	
	
		
			Vitamin Egészség- és Önsegélyező Pénztár
		
		
			18085246141
		
		
			1023 
		
		
			Budapest
		
		
			Bécsi út 4.
		
		
			
		
		
			
		
	


$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table').parent().scroll(function(){
	$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table').find('tr:first-child th div').css('top',$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table').parent().scrollTop()+'px');
})

function _b9c65da7f2ed5975a22138e065efc2a5_scroll() {
	$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table').find('tr:first-child th div').css('top',$('#_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table').parent().scrollTop()+'px');
}

components['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table']={&quot;data&quot;:[&quot;$client&quot;,&quot;listPenztar&quot;,&quot;PenztarList&quot;,{&quot;Aktiv&quot;:1,&quot;dbName&quot;:&quot;B9019003&quot;}],&quot;metadata&quot;:{&quot;data-id&quot;:&quot;$PenztarID&quot;,&quot;data-name&quot;:&quot;$Nev&quot;,&quot;data-iranyitoszam&quot;:&quot;$Iranyitoszam&quot;,&quot;data-telepules&quot;:&quot;$Telepules&quot;,&quot;data-cim&quot;:&quot;$Cim&quot;,&quot;data-adoszam&quot;:&quot;$Adoszam&quot;,&quot;data-aktiv&quot;:&quot;$Aktiv&quot;},&quot;datatype&quot;:{&quot;Nev&quot;:{&quot;header&quot;:&quot;N\u00e9v&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;,&quot;data-required&quot;:true},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Adoszam&quot;:{&quot;header&quot;:&quot;Ad\u00f3sz\u00e1m&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Iranyitoszam&quot;:{&quot;header&quot;:&quot;Ir\u00e1ny\u00edt\u00f3sz\u00e1m&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Telepules&quot;:{&quot;header&quot;:&quot;Telep\u00fcl\u00e9s&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Cim&quot;:{&quot;header&quot;:&quot;C\u00edm&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Aktiv&quot;:{&quot;header&quot;:&quot;Akt\u00edv&quot;,&quot;filter&quot;:&quot;bool&quot;,&quot;attributes&quot;:{&quot;data-type&quot;:&quot;checkbox&quot;}}},&quot;controlsAfter&quot;:{&quot;edit bb-green&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f_edit(this)&quot;},&quot;controlTitles&quot;:{&quot;edit bb-green&quot;:&quot;Szerkeszt\u00e9s&quot;},&quot;settings&quot;:{&quot;sortable&quot;:true,&quot;editable&quot;:true},&quot;_component&quot;:&quot;table&quot;,&quot;_hashname&quot;:&quot;_b9c65da7f2ed5975a22138e065efc2a5&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table&quot;};componentsLoaded['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table']={};Object.assign(componentsLoaded['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table'], components['_342edbf3b5a0d7a1bf4b8cdd1dfca70f-table']);
components['egeszsegpenztarak']={&quot;client&quot;:&quot;$client&quot;,&quot;read&quot;:{&quot;method&quot;:&quot;listPenztar&quot;,&quot;dataset&quot;:&quot;PenztarList&quot;,&quot;params&quot;:{&quot;Aktiv&quot;:1}},&quot;create&quot;:&quot;save-epenztar&quot;,&quot;update&quot;:&quot;save-epenztar&quot;,&quot;editable&quot;:[&quot;Nev&quot;,&quot;Iranyitoszam&quot;,&quot;Telepules&quot;,&quot;Cim&quot;,&quot;Adoszam&quot;,&quot;Aktiv&quot;],&quot;defaults&quot;:{&quot;PenztarID&quot;:&quot;-1&quot;,&quot;Nev&quot;:&quot;&quot;,&quot;Iranyitoszam&quot;:&quot;&quot;,&quot;Telepules&quot;:&quot;&quot;,&quot;Cim&quot;:&quot;&quot;,&quot;Adoszam&quot;:&quot;&quot;},&quot;data&quot;:{&quot;data-id&quot;:&quot;$PenztarID&quot;,&quot;data-name&quot;:&quot;$Nev&quot;,&quot;data-iranyitoszam&quot;:&quot;$Iranyitoszam&quot;,&quot;data-telepules&quot;:&quot;$Telepules&quot;,&quot;data-cim&quot;:&quot;$Cim&quot;,&quot;data-adoszam&quot;:&quot;$Adoszam&quot;,&quot;data-aktiv&quot;:&quot;$Aktiv&quot;},&quot;dataType&quot;:{&quot;Nev&quot;:{&quot;header&quot;:&quot;N\u00e9v&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;,&quot;data-required&quot;:true},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Adoszam&quot;:{&quot;header&quot;:&quot;Ad\u00f3sz\u00e1m&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Iranyitoszam&quot;:{&quot;header&quot;:&quot;Ir\u00e1ny\u00edt\u00f3sz\u00e1m&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Telepules&quot;:{&quot;header&quot;:&quot;Telep\u00fcl\u00e9s&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Cim&quot;:{&quot;header&quot;:&quot;C\u00edm&quot;,&quot;sort&quot;:&quot;alpha&quot;,&quot;filter&quot;:&quot;&quot;,&quot;attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;,&quot;data-type&quot;:&quot;text&quot;},&quot;header-attributes&quot;:{&quot;class&quot;:&quot;text-left&quot;}},&quot;Aktiv&quot;:{&quot;header&quot;:&quot;Akt\u00edv&quot;,&quot;filter&quot;:&quot;bool&quot;,&quot;attributes&quot;:{&quot;data-type&quot;:&quot;checkbox&quot;}}},&quot;_component&quot;:&quot;crud&quot;,&quot;_hashname&quot;:&quot;_342edbf3b5a0d7a1bf4b8cdd1dfca70f&quot;,&quot;_attributes&quot;:[],&quot;_name&quot;:&quot;egeszsegpenztarak&quot;};componentsLoaded['egeszsegpenztarak']={};Object.assign(componentsLoaded['egeszsegpenztarak'], components['egeszsegpenztarak']);
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;egeszsegpenztarak-modal&quot;)/div[@class=&quot;inner widerr&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='egeszsegpenztarak-modal']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Mentés'])[2]/following::div[3]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Számlaszám'])[2]/following::div[6]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[7]/div/div</value>
   </webElementXpaths>
</WebElementEntity>

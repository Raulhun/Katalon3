import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

//WebUI.callTestCase(findTestCase('BEMUTATO_onalloan_futtathatok/TC-Login'), null, FailureHandling.STOP_ON_FAILURE)
WebUI.click(findTestObject('PacienstorzsSearch/btnUjPaciens'))

WebUI.selectOptionByValue(findTestObject('PaciensFelvetelForm/selectAzonTipus'), '2', false)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Azonosito'), var_azonosito)

switch (var_statusExpectedResult.toString()) {
    case 'taj_hibas':
        'Ellenőrzi, hogy valóban hibás-e a TAJ'
        WebUI.waitForElementVisible(findTestObject('Alert_Messages/div_Hibs TAJ szm'), 8, FailureHandling.OPTIONAL)

        def errorMess = WebUI.getText(findTestObject('Alert_Messages/div_Hibs TAJ szm'))

        CharSequence seq = 'Hibás TAJ szám'

        if (WebUI.verifyEqual(errorMess.contains(seq), true)) {
            KeywordUtil.markWarning(('A ' + var_azonosito) + ' TAJ tényleg hibas!')
        }
        
		WebUI.delay(3)
		
		if (WebUI.verifyElementClickable(findTestObject('Alert_Messages/div_OK'))) {

			WebUI.click(findTestObject('Alert_Messages/div_OK'))
		}
		
        break
    case 'taj_letezik':
        'Ellenőrzi, hogy szerepel-e már a TAJ?'
        WebUI.waitForElementVisible(findTestObject('Alert_Messages/Alert_msg_box'), 8, FailureHandling.OPTIONAL)

        def errorMess = WebUI.getText(findTestObject('Alert_Messages/Alert_msg_box'))

        CharSequence seq = 'A páciens már szerepel'

        if (WebUI.verifyEqual(errorMess.contains(seq), true)) {
            KeywordUtil.markWarning(('A ' + var_azonosito) + ' TAJ szerepel az adatbázisban!')
        }
        
        if (WebUI.verifyElementClickable(findTestObject('Alert_Messages/div_OK'))) {
            KeywordUtil.markWarning('A hibaüzenet bezárható')
			WebUI.click(findTestObject('Alert_Messages/div_OK'))
        }
        
        break
    default:
        break
}

WebUI.click(findTestObject('PaciensFelvetelForm/btnPcienstrzsBack'))

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/input_Keress_search'), 20)


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
import org.openqa.selenium.Keys as Keys
import static com.kms.katalon.core.testobject.ConditionType.EQUALS
import org.openqa.selenium.WebDriver
import org.openqa.selenium.WebElement
import com.kms.katalon.core.webui.driver.DriverFactory


//WebUI.callTestCase(findTestCase('BEMUTATO_onalloan_futtathatok/TC-Login'), null, FailureHandling.STOP_ON_FAILURE)
WebUI.click(findTestObject('PacienstorzsSearch/btnUjPaciens'))

WebUI.setText(findTestObject('PaciensFelvetelForm/input_SzuletesiDatum'), var_szulDat)

WebUI.sendKeys(findTestObject('PaciensFelvetelForm/input_SzuletesiDatum'), Keys.chord(Keys.TAB))

switch (var_statusExpectedResult.toString()) {
    case 'szuldat_datum_ok':
        'Csekkoljuk, hogy egy jó szuldatot elfogad-e?'
        WebUI.delay(5)

        break
    case 'szuldat_nem_valos':
        'Csekkoljuk, hogy mi történik, ha a szüldeat hibás'
        WebUI.setText(findTestObject('PaciensFelvetelForm/input_Nev'), 'nev')

        WebUI.selectOptionByValue(findTestObject('PaciensFelvetelForm/selectAzonTipus'), '2', false)

        WebUI.setText(findTestObject('PaciensFelvetelForm/input_Azonosito'), '888888888')

        WebUI.delay(4)

        if (WebUI.waitForElementVisible(findTestObject('Alert_Messages/div_OK'), 4, FailureHandling.OPTIONAL)) {
            WebUI.click(findTestObject('Alert_Messages/div_OK'))
        }
        
        WebUI.setText(findTestObject('PaciensFelvetelForm/input_Iranyitoszam'), 'irsz')

        WebUI.setText(findTestObject('PaciensFelvetelForm/input_Telepules'), 'telep')

        WebUI.setText(findTestObject('PaciensFelvetelForm/input_Cim'), 'cim')

        WebUI.click(findTestObject('PaciensFelvetelForm/btnMentes'))
		
		WebUI.delay(2)
		
		'webdrivert csinalunk'
		WebDriver driver = DriverFactory.getWebDriver()
		
		'megkeressük melyik elemen van a cursor'
		WebElement focusedElement = driver.switchTo().activeElement()
		
		'megnézzük, hogy a focusban lévő mezőnek van-e SzuletesiDatum attributuma'
		if (focusedElement.getAttribute('name') == 'SzuletesiDatum') {
			KeywordUtil.markPassed("Valóban hibás a szuletesi datum mező, mentés nem sikerült, focus a Szuldat mezőben van.")
		} else {
			KeywordUtil.markError("Nem a szuldat mezőben van a focus")
		}
        
        break
    default:
        break
}

WebUI.click(findTestObject('PaciensFelvetelForm/btnPcienstrzsBack'))

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/input_Keress_search'), 20)


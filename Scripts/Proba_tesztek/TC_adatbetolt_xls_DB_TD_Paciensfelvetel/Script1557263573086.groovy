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
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.WebDriver as WebDriver
import com.kms.katalon.core.testobject.ConditionType as ConditionType

WebUI.callTestCase(findTestCase('BEMUTATO_onalloan_futtathatok/TC-Login'), null, FailureHandling.STOP_ON_FAILURE)

//klikk a paciensfelv gombra
if (WebUI.verifyElementVisible(findTestObject('PacienstorzsSearch/btnUjPaciens'))) {
    WebUI.click(findTestObject('PacienstorzsSearch/btnUjPaciens'))
} else {
}

//adatok beadasa tesztData-ból
for (def row = 1; row <= findTestData('TD-Paciensfelvetel-adatbevitel-multi').getRowNumbers(); row++) {
    WebUI.setText(findTestObject('PaciensFelvetelForm/input_Nev'), findTestData('TD-Paciensfelvetel-adatbevitel-multi').getValue(
            'nev', row))

    WebUI.selectOptionByValue(findTestObject('PaciensFelvetelForm/selectAzonTipus'), '2', false)

    //findTestData('TD-Paciensfelvetel-adatbevitel-multi').getValue('azontipus', row)
    WebUI.setText(findTestObject('PaciensFelvetelForm/input_Azonosito'), findTestData('TD-Paciensfelvetel-adatbevitel-multi').getValue(
            'azonosito', row))

    //a letezeesellenőrzés leokézása, ha van
    boolean verifyMsgbox = true

    verifyMsgbox = WebUI.waitForElementVisible(findTestObject('Alert_Messages/Alert_msg_box'), 3, FailureHandling.OPTIONAL)

    if (verifyMsgbox) {
        WebUI.click(findTestObject('Alert_Messages/div_OK'))
    }
    
    //tovabbb az adatbevitel
    WebUI.setText(findTestObject('PaciensFelvetelForm/input_SzuletesiDatum'), findTestData('TD-Paciensfelvetel-adatbevitel-multi').getValue(
            'szulDat', row))

    WebUI.setText(findTestObject('PaciensFelvetelForm/input_Iranyitoszam'), findTestData('TD-Paciensfelvetel-adatbevitel-multi').getValue(
            'irsz', row))

    WebUI.setText(findTestObject('PaciensFelvetelForm/input_Cim'), findTestData('TD-Paciensfelvetel-adatbevitel-multi').getValue(
            'cím', row))

    //mentes
    WebUI.click(findTestObject('PaciensFelvetelForm/btnMentes'))

    //várunk míg betöltődik az oldal
    WebUI.waitForElementPresent(findTestObject('PacienstorzsSearch/input_Keress_search'), 10)

    if (row < findTestData('TD-Paciensfelvetel-adatbevitel-multi').getRowNumbers()) {
        //click az új páciens gombra
        WebUI.click(findTestObject('PacienstorzsSearch/btnUjPaciens'))
    }
}
WebUI.closeBrowser()


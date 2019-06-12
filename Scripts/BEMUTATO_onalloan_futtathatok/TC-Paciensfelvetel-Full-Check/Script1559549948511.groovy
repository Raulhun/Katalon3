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
import org.openqa.selenium.By as By
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.annotation.Keyword as Keyword
import org.openqa.selenium.WebDriver as WebDriver
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('BEMUTATO_onalloan_futtathatok/TC-Login'), null, FailureHandling.STOP_ON_FAILURE)

// ------------------megnezzük szerepel-e már a páciens a törzsben------------------------------
//beadjuk a keresőbe az azonosítót
if (var_azonosito != '') {
    WebUI.setText(findTestObject('PacienstorzsSearch/input_Keress_search'), var_azonosito)
} else {
    KeywordUtil.markFailedAndStop('  Az azonosito nincs kitöltve!  ')

    WebUI.closeBrowser()
}

//várunk az eredményre
WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_Teljes lista'), 20)

WebDriver driver = DriverFactory.getWebDriver()

//egy listába írjuk a találat sorait
List<WebElement> TotalCollCount = driver.findElements(By.xpath('//table[@id=\'patients-table\']/tbody/tr[2]/td'))

if (TotalCollCount.size() == 8) {
    var_noinput_azonElofordul = true

    KeywordUtil.markWarning(('A következő TAJ: ' + var_azonosito) + ' szerepel a törzsben!')
} else {
    var_noinput_azonElofordul = false

    KeywordUtil.markWarning(('A következő TAJ: ' + var_azonosito) + ' NEM szerepel a törzsben!')
}

// ------------------ pacienselofordulas vizsgalat vege ------------------------------------
//------------------- adatbevitel kezdete --------------------------------------------------
WebUI.click(findTestObject('PacienstorzsSearch/btnUjPaciens'))

WebUI.delay(3)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Nev'), var_nev)

WebUI.selectOptionByValue(findTestObject('PaciensFelvetelForm/selectAzonTipus'), '2', false)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Azonosito'), var_azonosito)

if (var_noinput_azonElofordul) {
    WebUI.waitForElementVisible(findTestObject('Alert_Messages/Alert_msg_box'), 8, FailureHandling.OPTIONAL)

    WebUI.click(findTestObject('Alert_Messages/div_OK'))
}

WebUI.setText(findTestObject('PaciensFelvetelForm/input_SzuletesiDatum'), var_szulDat)

WebUI.sendKeys(findTestObject('PaciensFelvetelForm/input_SzuletesiDatum'), Keys.chord(Keys.TAB))

WebUI.setText(findTestObject('PaciensFelvetelForm/input_SzuletesiHely'), var_szulHely)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_AnyjaNeve'), var_anyjaNeve)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_SzuletesiNev'), var_szulNev)

WebUI.selectOptionByValue(findTestObject('PaciensFelvetelForm/select_Allampolg'), var_allampolg, false)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Telefonszam'), var_tel)

def mobileSzamAtalkitott = (((var_mobile.toString().substring(0, 2) + '/') + var_mobile.toString().substring(2, 5)) + '-') + 
var_mobile.toString().substring(5, 9)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Mobiltelefon'), mobileSzamAtalkitott)

WebUI.sendKeys(findTestObject('PaciensFelvetelForm/input_Mobiltelefon'), Keys.chord(Keys.TAB))

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Iranyitoszam'), var_irsz)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Telepules'), var_telep)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Cim'), var_cim)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_Email'), var_email)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_SzemlyiIgSzama'), var_szig)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_KozgyogyKezdete'), var_kgyogyKezd)

WebUI.setText(findTestObject('PaciensFelvetelForm/input_KozgyogyVege'), var_kgyogyVege)

WebUI.sendKeys(findTestObject('PaciensFelvetelForm/input_KozgyogyVege'), Keys.chord(Keys.TAB))

WebUI.setText(findTestObject('PaciensFelvetelForm/input_KozgyogyIgSzama'), var_kgyogyIgSzam)

WebUI.click(findTestObject('PaciensFelvetelForm/btnMentes'))

WebUI.waitForElementPresent(findTestObject('PacienstorzsSearch/input_Keress_search'), 20)

//---------------------------------------- Adatbevitel vége -------------------------------------------
//---------------------------------------- Test-eset kiétékelésének kezdete ----------------------------
KeywordUtil.markWarning('-> 1. eset Sikeres mentés üzenet jön-e?')

def boolMsg = WebUI.waitForElementVisible(findTestObject('Alert_Messages/div_Sikeres ments'), 8, FailureHandling.OPTIONAL)

def errorMess = WebUI.getText(findTestObject('Alert_Messages/div_Sikeres ments'))

CharSequence seq = 'Sikeres mentés'

if (WebUI.verifyEqual(errorMess.contains(seq), true)) {
    KeywordUtil.markWarning(('Jön a ' + errorMess) + ' alert')
} else {
    KeywordUtil.markWarning(('Jön a ' + errorMess) + ' alert')
}

//------------------------------------------------------------------------------
KeywordUtil.markWarning('-> 2. eset Automatikusan kiválastásra kerül-e az újonann felvett páciens?')

def headerPatient = driver.findElement(By.xpath('//div[@class=\'inline patient\']/span')).getText()

KeywordUtil.markWarning('A headerPatient erteke: ' + headerPatient)

CharSequence seqNev = var_nev

if (WebUI.verifyEqual(headerPatient.contains(seqNev), true)) {
    KeywordUtil.markWarning('A páciens automatikusan kiválasztásra került a header-ben.')
}

//---------------------------------------------------------------------------------
KeywordUtil.markWarning('-> 3. eset A Cookie-ban a jó páciens ID szerepel')

def CookieUserHash = driver.manage().getCookieNamed('userhash').getValue()

def patientKey = CookieUserHash + '-patient'

def patientID = driver.manage().getCookieNamed(patientKey).getValue()

KeywordUtil.markWarning('A páciens ID a Cookie-ban: ' + patientID)

WebUI.setText(findTestObject('PacienstorzsSearch/input_Keress_search'), var_azonosito)

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_Teljes lista'), 20)

List<WebElement> TotalRowCount = driver.findElements(By.xpath('//table[@id=\'patients-table\']/tbody/tr'))

def countResults = TotalRowCount.size() - 2

KeywordUtil.markWarning('A találatok száma: ' + countResults)

def value = ''

int j = 0

for (WebElement row : TotalRowCount) {
    value = row.getAttribute('data-value')

    if (value == patientID) {
        KeywordUtil.markWarning(('A/Az ' + patientID) + ' ugyanaz a Cookie-ban mint a kiválasztott páciensnél!')
    }
}

/**
for(WebElement row : TotalRowCount){
	
	j++
	
	KeywordUtil.markWarning('A j értéke: ' + j)
	
	value = row.getAttribute("data-value")
	WebElement kivalSor
	
	if (j>1 || j <= countResults+1){
		KeywordUtil.markWarning('Beléptem')
		kivalSor = driver.findElement(By.xpath("//table[@id='patients-table']/tbody/tr[" + (j) + "]/td[1]/i[1]"))
	}
	
	if(kivalSor.getAttribute("class") == "fa fa-dot-circle-o") {
		KeywordUtil.markWarning('value: ' + value + '--------- ID: ' + patientID)
		if (value == patientID){
			KeywordUtil.markWarning('A/Az ' + patientID + ' ugyanaz a Cookie-ban mint a kiválasztott páciensnél!')
		}
	}
}
*/
//---------------------------------------------------------------------------------------
KeywordUtil.markWarning('-> 4. eset A felvett adatok mentése ténylegesen megtörtént')

WebUI.delay(3)

WebUI.waitForElementClickable(findTestObject('PacienstorzsSearch/btn_Pciens szerkesztse'), 8)

WebUI.click(findTestObject('PacienstorzsSearch/btn_Pciens szerkesztse'))

WebUI.delay(3)

WebUI.verifyEqual(var_nev, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_Nev'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_azonosito, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_Azonosito'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_szulDat, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_SzuletesiDatum'), 'value'), 
    FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_szulHely, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_SzuletesiHely'), 'value'), 
    FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_szulNev, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_SzuletesiNev'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_anyjaNeve, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_AnyjaNeve'), 'value'), FailureHandling.OPTIONAL)

//WebUI.verifyEqual(var_allampolg, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/select_Allampolg'), 'value'), FailureHandling.OPTIONAL)
WebUI.verifyEqual(var_tel, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_Telefonszam'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(mobileSzamAtalkitott, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_Mobiltelefon'), 'value'), 
    FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_irsz, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_Iranyitoszam'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_telep, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_Telepules'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_cim, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_Cim'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_email, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_Email'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_szig, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_SzemlyiIgSzama'), 'value'), FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_kgyogyKezd, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_KozgyogyKezdete'), 'value'), 
    FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_kgyogyVege, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_KozgyogyVege'), 'value'), 
    FailureHandling.OPTIONAL)

WebUI.verifyEqual(var_kgyogyIgSzam, WebUI.getAttribute(findTestObject('PaciensFelvetelForm/input_KozgyogyIgSzama'), 'value'), 
    FailureHandling.OPTIONAL)

WebUI.click(findTestObject('PaciensFelvetelForm/btnPcienstrzsBack'))


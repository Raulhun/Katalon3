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

WebUI.callTestCase(findTestCase('BEMUTATO_onalloan_futtathatok/TC-Login'), null, FailureHandling.STOP_ON_FAILURE)

//Az eredménytábla adatai
//beadjuk a keresőbe az azonosítót
WebUI.setText(findTestObject('PacienstorzsSearch/input_Keress_search'), GlobalVariable.azonFromData)

//várunk az eredményre
WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_Teljes lista'), 20)

WebDriver driver = DriverFactory.getWebDriver()

WebElement Webtable = driver.findElement(By.id('patients-table'))

//egy listába írjuk a találat sorait
List<WebElement> TotalRowCount = Webtable.findElements(By.xpath('//table[@id=\'patients-table\']/tbody/tr'))

int CountResults = TotalRowCount.size() - 2 //talállatok száma

KeywordUtil.markWarning('A találatok száma: ' + CountResults)

//Ha csak 1 td van vagyis csak 1 oszlop a 2. sorban, akkor az azt jelenti , hogy nincs találat

int k = 2;

for (int i = 1; i < (CountResults + 1); i++) {
	
	KeywordUtil.markWarning('A k erteke: ' + k)
	
    WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_Kereses'), 20)

    WebUI.setText(findTestObject('PacienstorzsSearch/input_Keress_search'), GlobalVariable.azonFromData)

    WebUI.delay(3)

    WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_Teljes lista'), 20)

    Webtable = driver.findElement(By.id('patients-table'))

    Webtable.findElement(By.xpath('//table[@id=\'patients-table\']/tbody/tr[' + k + ']/td[8]/i[2]')).click()

    Webtable.findElement(By.xpath('//table[@id=\'patients-table\']/tbody/tr[' + k + ']/td[8]/div/i[5]')).click()
	
    //--------------------------------
    WebUI.waitForAlert(20)

    WebUI.delay(2)

    WebUI.setAlertText('törlés')

    WebUI.acceptAlert()

	if (WebUI.waitForElementVisible(findTestObject('Alert_Messages/div_torles_nem_lehetseges'), 8)) {
		WebUI.click(findTestObject('Alert_Messages/div_OK_torl_nem_lehetseges'))
		k = k + 1
	}
    
    WebUI.delay(4)
}


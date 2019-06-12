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

WebDriver driver = DriverFactory.getWebDriver()

driver.findElement(By.xpath('//table[@id=\'patients-table\']/tbody/tr[2]/td[3]')).click()

WebUI.delay(3)

WebUI.waitForElementClickable(findTestObject('PacienstorzsSearch/btn_Pciens szerkesztse'), 8)

WebUI.click(findTestObject('PacienstorzsSearch/btn_Pciens szerkesztse'))

WebUI.delay(3)

inputNev = WebUI.getText(findTestObject('PaciensFelvetelForm/input_Nev'))

WebUI.verifyEqual(WebUI.getText(findTestObject('PaciensFelvetelForm/input_Nev')), inputNev)




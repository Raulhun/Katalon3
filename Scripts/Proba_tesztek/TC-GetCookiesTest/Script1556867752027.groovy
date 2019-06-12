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
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import org.openqa.selenium.By as By

WebUI.callTestCase(findTestCase('BEMUTATO_onalloan_futtathatok/TC-Login'), null, FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/input_Keress_search'), 20)

WebDriver driver = DriverFactory.getWebDriver()

driver.findElement(By.xpath("//table[@id='patients-table']/tbody/tr[2]/td[3]")).click()

def CookieUserID = driver.manage().getCookieNamed('userid').getValue()

def CookieDb = driver.manage().getCookieNamed('db').getValue()

def CookieUserHash = driver.manage().getCookieNamed('userhash').getValue()

def patientKey = CookieUserHash + '-patient'

def patientID = driver.manage().getCookieNamed(patientKey).getValue()

KeywordUtil.markWarning('A páciens ID a Cookie-ban: ' + patientID)

println('----------usehash: ' + CookieUserHash)

println('----------patientkey: ' + patientKey)

//String CookiePatient = driver.manage().getCookieNamed('054f4098-page').getValue();
println('--------------- USER ID: ' + CookieUserID)

println('--------------- BD: ' + CookieDb)

// println('--------------- PATIENT: ' + CookiePatient);
if (CookieUserID == '21') {
    println('-----------yep')
}




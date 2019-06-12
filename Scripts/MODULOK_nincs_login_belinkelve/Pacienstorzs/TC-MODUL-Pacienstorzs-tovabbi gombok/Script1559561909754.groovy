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
import org.openqa.selenium.interactions.Actions as Actions

//Letrehozunk egy webdrivert
WebDriver driver = DriverFactory.getWebDriver()

// Create object of Action class
Actions action = new Actions(driver)

//egy paciens kivalasztasa
WebElement Webtable = driver.findElement(By.id('patients-table'))

Webtable.findElement(By.xpath('//table[@id=\'patients-table\']/tbody/tr[2]/td[3]')).click()

WebUI.delay(3)

//Bal felso kis gombok clickelese
//1. Elojegyzes 1
WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_elojegyzes_1'), 10)

WebUI.waitForElementClickable(findTestObject('PacienstorzsSearch/btn_elojegyzes_1'), 10)

WebUI.click(findTestObject('PacienstorzsSearch/btn_elojegyzes_1'))

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_Teljes lista'), 10)

WebUI.waitForElementClickable(findTestObject('PacienstorzsSearch/btn_Teljes lista'), 10)

WebUI.click(findTestObject('PacienstorzsSearch/btn_Teljes lista'))

WebUI.delay(3)

//2. Elojegyzes 2
WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_elojegyzes_2'), 10)

WebUI.waitForElementClickable(findTestObject('PacienstorzsSearch/btn_elojegyzes_2'), 10)

WebUI.click(findTestObject('PacienstorzsSearch/btn_elojegyzes_2'))

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_Teljes lista'), 10)

WebUI.waitForElementClickable(findTestObject('PacienstorzsSearch/btn_Teljes lista'), 10)

WebUI.click(findTestObject('PacienstorzsSearch/btn_Teljes lista'))

WebUI.delay(3)

//3. Paciens statusz
WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/btn_statusz_1'), 10)

WebUI.waitForElementClickable(findTestObject('PacienstorzsSearch/btn_statusz_1'), 10)

WebUI.click(findTestObject('PacienstorzsSearch/btn_statusz_1'))

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/modal_paciens_status'), 10)

// Sendkeys using Action class object
action.sendKeys(Keys.ESCAPE).build().perform()

WebUI.delay(1)

//Sorvégi kis ikonok nyomkodása
//1.páciens szerkesztése
WebUI.click(findTestObject('PacienstorzsSearch/btn_sorvegi_patientedit'))

WebUI.waitForElementVisible(findTestObject('PaciensFelvetelForm/btnPcienstrzsBack'), 10)

WebUI.click(findTestObject('PaciensFelvetelForm/btnPcienstrzsBack'))

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/input_Keress_search'), 10)

//2. paciens szamlai
WebUI.click(findTestObject('PacienstorzsSearch/sorvegi_gombok/btn_sorvegi_popupmenu'))

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/sorvegi_gombok/btn_szamlazas_1'), 10)

WebUI.click(findTestObject('PacienstorzsSearch/sorvegi_gombok/btn_szamlazas_1'))

if (WebUI.verifyElementVisible(findTestObject('PacienstorzsSearch/sorvegi_gombok/modal_szamlak_1'))) {
    action.sendKeys(Keys.ESCAPE).build().perform()
}

if (WebUI.verifyElementVisible(findTestObject('Alert_Messages/Alert_msg_box'))) {
    WebUI.click(findTestObject('Alert_Messages/div_OK'))
}


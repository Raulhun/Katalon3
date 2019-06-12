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

'Letrehozunk egy webdrivert'
WebDriver driver = DriverFactory.getWebDriver()

'Create object of Action class'
Actions action = new Actions(driver)

'Belepes a beallitasok menube'
WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_sidebaron'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/btn_Mentes_Bealitasokban'), 10)

WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_Vezerlopult_sidebaron'))

'1. Teszt recept btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_Teszt recept_Beallitasokban'))

WebUI.waitForPageLoad(10)

WebUI.switchToWindowIndex(0)

'2. Tovabbi fiokok btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_Tovbbi_fiokok_Beallitasokban'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/modal_szamlazz_fiokok_Beallitasokban'), 10)

WebUI.delay(2)

action.sendKeys(Keys.ESCAPE).build().perform()

WebUI.delay(2)

'3. Egeszsegpenztarak btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_Egeszsegpenztarak_Beallitasokban'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/modal_egeszsegpenztarak_Beallitasokban'), 10)

WebUI.delay(2)

action.sendKeys(Keys.ESCAPE).build().perform()

WebUI.delay(2)

'4. Kedvezmenyezett csoportok btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_Kedvezmnyezett csoportok_Beallitasokban'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/input_kedvezm_csop'), 10)

WebUI.delay(2)

WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_sidebaron'))

'5. email sablon szovegek btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_Email sablonszvegek'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/modal_email_sablon'), 10)

WebUI.delay(2)

action.sendKeys(Keys.ESCAPE).build().perform()

WebUI.delay(2)

'6. Online elojegyzes btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_Online _eljegyzs_belltsai'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/input_online_elojegyzesek'), 10)

WebUI.delay(2)

WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_sidebaron'))

'7. cegek telephelyei btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_cegek_telephelyek'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/btn_uj_ceg_Cegek_Telephelyeken'), 10)

WebUI.delay(2)

WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_sidebaron'))

'8. Alkalmassagi vizsgalat adatai btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_alkalmassagi_vizsg_adatai'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/select_Alaklmassagi_vizsgalatban'), 10)

WebUI.delay(2)

WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_sidebaron'))

'9, Paciens statusok btn click'
WebUI.click(findTestObject('Vezerlopult/Beallitasok/btn_paciens_statusok'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/modal_status_sablonok'), 10)

WebUI.delay(2)

action.sendKeys(Keys.ESCAPE).build().perform()

WebUI.delay(2)


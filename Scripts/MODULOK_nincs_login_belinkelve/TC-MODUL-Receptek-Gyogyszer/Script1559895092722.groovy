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

'letrehozzuk a drivereket és az actiont'
WebDriver driver = DriverFactory.getWebDriver()

WebDriver driver3 = DriverFactory.getWebDriver()

Actions action = new Actions(driver3)

'kivalasztunk egy pacienst'
WebElement Webtable = driver.findElement(By.id('patients-table'))

Webtable.findElement(By.xpath('//table[@id=\'patients-table\']/tbody/tr[2]/td[3]')).click()

WebUI.delay(2)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Receptek_sidebar'))

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Gyogyszer_Receptek'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/page_Gyogyszer'), 30)

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/btn_Uj_recept_Gyogyszer'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Uj_recept_Gyogyszer'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/page_Uj_Recept'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Receptek_back_Uj_recept'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/page_Gyogyszer'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Uj_recept_Gyogyszer'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/page_Uj_Recept'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/td_gyogyszer_felvesz'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/btn_BNO_search'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_BNO_search'))

WebUI.delay(2)

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/input_Keress_search_Uj_recept'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/td_2_BNO_ modal'))

WebUI.setText(findTestObject('H_Receptek/Gyogyszer/input_Adagolas'), '1x1')

WebUI.delay(2)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Mentes'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/alert_sikeres_mentes'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/OK_alert_sikeres_mentes'))

WebUI.waitForPageLoad(60)

WebUI.switchToWindowIndex(0)

WebUI.closeWindowIndex(1)

WebUI.switchToWindowIndex(0)

WebUI.delay(1)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Receptek_sidebar'))

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Gyogyszer_Receptek'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/page_Gyogyszer'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Szerkesztes'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/btn_BNO_search'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Receptek_back_Uj_recept'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/alert_sikeres_mentes'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/OK_alert_sikeres_mentes'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/page_Gyogyszer'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Nyomtatas'))

WebUI.delay(3)

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/span_1_peldany'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/span_1_peldany'))

WebUI.waitForPageLoad(60)

WebUI.switchToWindowIndex(0)

WebUI.closeWindowIndex(1)

WebUI.switchToWindowIndex(0)

WebUI.delay(1)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Receptek_sidebar'))

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Gyogyszer_Receptek'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/page_Gyogyszer'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/btn_Torles'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/alert_sikeres_mentes'), 30)

WebUI.click(findTestObject('H_Receptek/Gyogyszer/OK_alert_sikeres_mentes'))

WebUI.waitForElementVisible(findTestObject('H_Receptek/Gyogyszer/btn_Uj_recept_Gyogyszer'), 30)

WebUI.click(findTestObject('H_Receptek/Magisztralis/btn_Magisztralis_sidebar'))

WebUI.verifyElementVisible(findTestObject('H_Receptek/Magisztralis/btn_Magisztralis_sidebar'))

WebUI.click(findTestObject('H_Receptek/Segedeszkoz/btn_Segedeszkoz_sidebar'))

WebUI.verifyElementVisible(findTestObject('H_Receptek/Segedeszkoz/btn_Segedeszkoz_sidebar'))


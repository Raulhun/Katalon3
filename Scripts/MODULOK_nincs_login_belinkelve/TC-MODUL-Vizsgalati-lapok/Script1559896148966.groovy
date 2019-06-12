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

'felvesszük a drivert, meg az actions-t'
WebDriver driver = DriverFactory.getWebDriver()

WebDriver driver3 = DriverFactory.getWebDriver()

Actions action = new Actions(driver3)

'kivalasztunk egy pacienst'
WebElement Webtable = driver.findElement(By.id('patients-table'))

Webtable.findElement(By.xpath('//table[@id=\'patients-table\']/tbody/tr[2]/td[3]')).click()

WebUI.delay(1)

'vizsgalati lap felvetele'
WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Vizsgalati_lapok_sidebar'))

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Rogzitett_vizsgalatok'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Uj_vizsgalati_lap_Rogzitett_vizsg'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Uj_vizsgalati_lap_Rogzitett_vizsg'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Mentes_Rogzitett_vizsgalatok'), 30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Vizsgalati_lapok_back'))

WebUI.waitForElementVisible(findTestObject('Alert_Messages/Alert_msg_box'), 30)

WebUI.click(findTestObject('Alert_Messages/div_OK'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Uj_vizsgalati_lap_Rogzitett_vizsg'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Uj_vizsgalati_lap_Rogzitett_vizsg'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Mentes_Rogzitett_vizsgalatok'), 30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_BNO_kod_hozaadasa'))

WebUI.delay(3)

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/input_Keress_search_BNO_kod_hozzaadasa'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/td_BNO_kod_hozaadasa'))

WebUI.delay(3)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Elozo_vizsgalati_lap'))

WebUI.delay(3)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Felirt_gyogyszerek'))

WebUI.delay(3)

WebUI.waitForElementClickable(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Mentes_Rogzitett_vizsgalatok'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Mentes_Rogzitett_vizsgalatok'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Veglegesites_szamlazas_modal'), 30)

WebUI.delay(3)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Veglegesites_szamlazas_modal'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Uj_vizsgalati_lap_Rogzitett_vizsg'), 
    30)

'Vizsgalati lap szerkesztes'
WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Szerkesztes_Rogzitett_vizsg'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Vizsgalati_lapok_back'), 30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Vizsgalati_lapok_back'))

WebUI.waitForElementVisible(findTestObject('Alert_Messages/Alert_msg_box'), 30)

WebUI.click(findTestObject('Alert_Messages/div_OK'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Szerkesztes_Rogzitett_vizsg'), 30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Szerkesztes_Rogzitett_vizsg'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Vizsgalati_lapok_back'), 30)

WebUI.delay(2)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Mentes_szerkesztes'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Nyomtatas_Rogzitett_vizsgalatok'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Nyomtatas_Rogzitett_vizsgalatok'))

WebUI.delay(3)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/li_Nyomtatas_almenu'))

WebUI.waitForPageLoad(60)

//WebUI.switchToWindowIndex(0)

WebUI.delay(7)

WebUI.closeWindowIndex(1)

WebUI.switchToWindowIndex(0)

'vizsgalati lap sablonra ugras'
WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Vizsgalati_lapok_sidebar'))

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Rogzitett_vizsgalatok'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Uj_vizsgalati_lap_Rogzitett_vizsg'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Sablonok_kezelese_Rogzitett_vizsgalatok'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Vizsgalati_lap_sablonok/btn_Uj_sablon_Vizsgalati_lap_sablonok'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Vizsgalati_lap_sablonok/btn_Vizsgalati_lapok_Vizsgalati_lap_sablonok'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Rogzitett_vizsgalatok/btn_Uj_vizsgalati_lap_Rogzitett_vizsg'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Vizsgalati_lap_sablonok/btn_Vizsgalati_lap_sablonok'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Vizsgalati_lap_sablonok/btn_Uj_sablon_Vizsgalati_lap_sablonok'), 
    30)

WebUI.click(findTestObject('Vizsgalati_lapok/Nyomtatasi_sablonok/btn_Nyomtatasi_sablonok_sidebar'))

WebUI.waitForElementVisible(findTestObject('Vizsgalati_lapok/Nyomtatasi_sablonok/btn_Uj_sablon'), 30)

WebUI.click(findTestObject('Vizsgalati_lapok/Citologiai_naplo/btn_Citologiai_naplo_sidebar'))

WebUI.delay(3)

WebUI.click(findTestObject('Vizsgalati_lapok/Nyomtatvanytar/btn_Nyomtatvanytar_sidebar'))

WebUI.delay(3)


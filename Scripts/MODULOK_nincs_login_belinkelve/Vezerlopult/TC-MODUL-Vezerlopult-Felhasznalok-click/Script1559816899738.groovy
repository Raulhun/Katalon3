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

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/btn_Mentes_Bealitasokban'), 30)

'1. Felhasznalok/Sajat felhasznalo gomb click'
WebUI.click(findTestObject('Vezerlopult/Felhasznalok/btn_Felhasznalok_Vezerlopult_sidebar'))

WebUI.click(findTestObject('Vezerlopult/Felhasznalok/btn_Sajat_felhasznalo_Vezerlopult_sidebar'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Felhasznalok/div_Mentes'), 30)

'2. Felhasznalok/Felhasznalok kezelese gomb click'
WebUI.click(findTestObject('Vezerlopult/Felhasznalok/btn_Felhasznalok_kezelese_Vezerlopult_sidebar'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Felhasznalok/btn_Uj_Felhasznalo'), 30)

'2/a. Felhasznalok/Felhasznalok kezelese/Új felhasználó gomb click'
WebUI.click(findTestObject('Vezerlopult/Felhasznalok/btn_Uj_Felhasznalo'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Felhasznalok/div_Mentes'), 30)

WebUI.click(findTestObject('Vezerlopult/Felhasznalok/btn_Felhasznalok_kezelese_Vezerlopult_sidebar'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Felhasznalok/btn_Felhasznalo_szerkesztese'), 30)

'2/b. Felhasznalok/Felhasznalok kezelese/Felhasznalo szerkesztese click'
WebUI.click(findTestObject('Vezerlopult/Felhasznalok/btn_Felhasznalo_szerkesztese'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Felhasznalok/div_Mentes'), 30)

'3. Felhasznalok/Jogosultsagok click'
WebUI.click(findTestObject('Vezerlopult/Felhasznalok/btn_Jogosultsagok_Vezerlopult_sidebar'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Felhasznalok/table_jogosultsagok'), 30)

